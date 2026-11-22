"""Build an offline, per-example review from the browser capture directories."""
import argparse
from collections import Counter, defaultdict
import html
import json
from pathlib import Path
import re

CROSSWALK = Path(__file__).resolve().parents[1] / "docs/showcase-mantine-examples.json"


def read(file):
    return json.loads(file.read_text(encoding="utf-8-sig"))


def normalized(label):
    return re.sub(r"[^a-z0-9]+", " ", label.lower()).strip()


def example_cell(section, folder):
    if section is None:
        return "<span>尚无配对</span>"
    image = section.get("image")
    title = html.escape(section["title"])
    fields = " · ".join(dict.fromkeys(section.get("fields", [])))
    if section.get("selectors"):
        fields = " · ".join(s["name"] for s in section["selectors"])
    image_link = ""
    if image:
        path = html.escape(f"{folder}/{image}", quote=True)
        image_link = f'<a href="{path}"><img loading="lazy" src="{path}" alt="{title}"></a>'
    panel = html.escape(fields) or ("配置面板" if section["panel"] else "")
    sizes = []
    for node in section.get("geometry", []):
        if node["tag"] in {"input", "select", "textarea", "button", "svg"}:
            width, height = node["box"][2:]
            item = f'{node["tag"]} {width:g}×{height:g}'
            if item not in sizes:
                sizes.append(item)
    geometry = " · ".join(sizes[:4])
    return f"<strong>{title}</strong><small>{panel}</small>{image_link}<small>{html.escape(geometry)}</small>"


def painted_bounds(path):
    """Bound visible text, strokes and fills relative to the screenshot, excluding its outside border."""
    from PIL import Image, ImageChops
    with Image.open(path) as opened:
        image = opened.convert("RGB")
    width, height = image.size
    if width < 6 or height < 6:
        return None
    interior = image.crop((2, 2, width - 2, height - 2))
    sample = interior.resize((32, 32), Image.Resampling.NEAREST)
    background = max(sample.getcolors(1024), key=lambda pair: pair[0])[1]
    channels = ImageChops.difference(interior, Image.new("RGB", interior.size, background)).split()
    difference = ImageChops.lighter(ImageChops.lighter(channels[0], channels[1]), channels[2])
    box = difference.point(lambda value: 255 if value >= 26 else 0).getbbox()
    if not box:
        return None
    left, top, right, bottom = box
    return dict(left=left+2, top=top+2, width=right-left, height=bottom-top,
                offsetX=round((left+right+4-width)/2, 1), frame=[width, height])


def compare_controls(reference, local):
    """Compare only unique, equally labelled controls within the selected previews."""
    def measured(section):
        groups = defaultdict(list)
        for node in section.get("geometry", []):
            label = (node.get("label") or "").strip()
            if label and node["tag"] in {"button", "input", "select", "textarea"}:
                groups[node["tag"], label.casefold()].append(node)
        return {key: nodes[0] for key, nodes in groups.items() if len(nodes) == 1}

    expected, actual = measured(reference), measured(local)
    rows = []
    for key in sorted(expected.keys() & actual.keys()):
        a, b = expected[key], actual[key]
        rows.append({"control": key[0], "name": a["label"], "mantineSize": a["box"][2:],
                     "localSize": b["box"][2:], "deltaWidth": round(b["box"][2] - a["box"][2], 1),
                     "deltaHeight": round(b["box"][3] - a["box"][3], 1)})
    return rows


def compare_icons(reference, local):
    def named(section):
        entries = section.get("icons", [])
        groups = defaultdict(list)
        for icon in entries:
            name = (icon.get("name") or "").strip().casefold()
            if name:
                groups[name].append(icon)
        return {key: icons[0] for key, icons in groups.items() if len(icons) == 1}

    before, after = named(reference), named(local)
    matches = [(before[name], after[name]) for name in sorted(before.keys() & after.keys())]
    if not matches and len(reference.get("icons", [])) == 1 and len(local.get("icons", [])) == 1:
        a, b = reference["icons"][0], local["icons"][0]
        if not a.get("name") or not b.get("name"):
            matches = [(a,b)]
    return [{"name": a.get("name") or b.get("name") or "单个图标按钮",
             "mantineIcon": a["icon"], "localIcon": b["icon"], "mantineButton": a["button"], "localButton": b["button"],
             "mantineOffset": a["offset"], "localOffset": b["offset"]} for a,b in matches]


def pairings(root):
    result = {}
    for file in ["chart-date-extension-pairs.json", "layout-example-pairs.json", "display-example-pairs.json",
                 "display-verified-final/example-verification.json", "display-verified-208/example-verification.json", "display-authoritative/example-verification.json", "display-latest/example-verification.json", "field-example-pairs.json",
                 "extensions-final/example-verification.json", "extensions-final-v5/example-verification.json",
                 "extensions-visual-enhanced/example-verification.json", "extensions-published/example-verification.json",
                 "extensions-authoritative/example-verification.json"]:
        if not (root / file).exists():
            continue
        for row in read(root / file):
            image = (row.get("referenceImage") or row.get("officialImage") or "").replace(chr(92), "/")
            ordinal = row.get("referenceOrdinalOnPage") or row.get("referenceIndex")
            if ordinal is None and re.search(r"/[0-9]+[.]png$", image):
                ordinal = int(Path(image).stem)
            if ordinal is None:
                continue
            state = row.get("comparison") or row.get("state") or row.get("status")
            section = row.get("localSection") or row.get("localAnchor")
            notes = dict.fromkeys(row[key] for key in ("difference", "visualDifference", "visualObservation") if row.get(key))
            review = row.get("visualReview") or {}
            if review.get("observation"):
                notes[review["observation"]] = None
            if review.get("substantialLayoutOrSampleDifferenceRemains"):
                notes["仍有已确认的明显视觉或内容差异。"] = None
            result[row["slug"], ordinal] = (section, state, " ".join(notes))
    if (root / "layout-panel-map.json").exists():
        for row in read(root / "layout-panel-map.json"):
            result.setdefault((row["slug"], row["index"]), (row["section"], "paired", None))
    return result


def compare_field_icons(reference, local):
    def by_side(section):
        groups = defaultdict(list)
        for icon in section.get("fieldIcons", []):
            groups[normalized(icon["label"]), icon["side"]].append(icon)
        return groups
    before, after = by_side(reference), by_side(local)
    matches = [(a[0], after[key][0]) for key, a in before.items()
               if len(a) == 1 and len(after.get(key, [])) == 1]
    if not matches and len(reference.get("fieldIcons", [])) == len(local.get("fieldIcons", [])) == 1:
        a, b = reference["fieldIcons"][0], local["fieldIcons"][0]
        if not a["label"] or not b["label"] or normalized(a["label"]) == normalized(b["label"]):
            matches.append((a,b))
    return [{"mantine":a, "local":b} for a,b in matches]


def verified_controls(root):
    controls = {}
    def add(slug, title, changes, file, key=None):
        flags = [(label, outcome) for label, outcome in changes]
        controls[slug, key or normalized(title)] = (flags, file.relative_to(root).as_posix())
    for name in ["reference-controls.json", "verified-fields/index.json", "verified-geometry-field-panels/index.json"]:
        file = root / name
        if file.is_file():
            for panel in read(file)["panels"]:
                add(panel["slug"], panel.get("title", ""), ((label, value.get("changed", False)) for label, value in panel.get("controls", {}).items()), file)
    for family in ["layout-panels-*", "display-verified*"]:
        for directory in sorted(root.glob(family), key=lambda p:p.stat().st_mtime):
            if family.startswith("layout"):
                file = directory / "report.json"
                if file.is_file():
                    for row in read(file).get("results", []):
                        if row.get("width", 0) > 600 and "fields" in row:
                            add(row["slug"], "", ((f["name"], f.get("changed",False)) for f in row["fields"]), file, row["upstreamIndex"])
            else:
                file = directory / "panel-verification.json"
                if file.is_file():
                    for row in read(file):
                        add(row["slug"], row["referenceTitle"], ((c.get("label",""),c.get("changed",False)) for c in row["panel"].get("checks",[])),file)
    for options in sorted(root.glob("layout-options-*/report.json"), key=lambda p: p.stat().st_mtime):
        for row in read(options)["report"]:
            add(row["slug"], "", ((field["field"], bool(field.get("pass"))) for field in row["items"]), options, row["section"])
    display = root / "display-latest/example-verification.json"
    if display.is_file():
        for row in read(display):
            panel = row.get("panel")
            if panel and panel.get("checks"):
                add(row["slug"], row["referenceTitle"], ((entry["label"], entry.get("changed", False))
                    for entry in panel["checks"]), display, row["localSection"])
    return controls


def selector_evidence(root):
    checks = defaultdict(list)
    layout_reports = sorted(root.glob("layout-styles-*/report.json"), key=lambda p: p.stat().st_mtime)
    layout = layout_reports[-1] if layout_reports else root / "layout-selectors-frozen/report.json"
    if not layout.is_file():
        layout = root / "layout-style-final/report.json"
    sources = [
        ("field-style-head-comparison/index.json", "input"),
        (("display-latest/selector-checks.json" if (root / "display-latest/selector-checks.json").is_file()
          else "display-verified-final/selectors.json"), "display"),
        (layout.relative_to(root).as_posix(), "layout"),
        ("selector-checks-carousel.json", "carousel"),
    ]
    for name, family in sources:
        path = root / name
        if not path.exists():
            continue
        data = read(path)
        if family == "input":
            for row in data["results"]:
                status = {"matched": "官方和本地均可高亮", "also-hidden-upstream": "两侧均未显示",
                          "local-additional": "仅本地显示"}.get(row["status"], "尚待核对")
                checks[row["slug"]].append((row["name"], status, name))
        if family == "display":
            if isinstance(data, dict):
                for row in data["records"]:
                    for control in row["controls"]:
                        ok = control.get("hover") and control.get("focus")
                        checks[row["slug"]].append((control["name"], "已高亮" if ok else "尚待核对", name))
            else:
                for row in data:
                    checks[row["slug"]].append((row["name"], "已高亮" if row["ok"] else "尚待核对", name))
        if family == "layout":
            for row in data if isinstance(data, list) else data.get("results", []):
                for field in row.get("fields", []):
                    if field.get("highlights", 0):
                        value = (field["name"], "已高亮", name)
                        if value not in checks[row["slug"]]:
                            checks[row["slug"]].append(value)
        if family == "carousel":
            for row in data["records"]:
                for field in row["controls"]:
                    status = "已高亮" if field["hover"] and field["focus"] else "尚待核对"
                    checks[row["slug"]].append((field["name"], status, name))
    compact = layout.with_name("pagination-compact-label.json")
    if compact.is_file():
        label = read(compact)
        if label.get("width", 0) > 0 and "solid" in label.get("outline", ""):
            checks["pagination"].append(("label", "窄容器已高亮", compact.relative_to(root).as_posix()))
    return checks


def action_proofs(root):
    actions = {}
    for folder in ("date-direct", "reference-variants", "range-and-segment-actions", "timeline-label-actions"):
        path = root / folder / "checks.json"
        if not path.exists():
            continue
        checks = read(path)
        for row in checks["results"]:
            if not row.get("passed"):
                continue
            slug, anchor = row["slug"], row["id"]
            paths = [root / folder / f"{anchor}-{state}.png" for state in ("before", "after")]
            if all(path.is_file() for path in paths):
                actions[slug, anchor] = dict(base=checks["base"], before=paths[0].relative_to(root).as_posix(),
                                             after=paths[1].relative_to(root).as_posix())
    return actions


def component(root, slug, totals, mapping, controls, selectors, actions, examples, measure_images, panel_captures, baseline):
    local = read(root / "local" / slug / "index.json")
    upstream = read(root / "upstream" / slug / "index.json")
    mobile = {}
    mobile_errors = []
    for side in ("local", "upstream"):
        index = root / "viewport-390" / side / slug / "index.json"
        if index.is_file():
            data = read(index)
            if side == "upstream" or data.get("asset") == local.get("asset"):
                mobile[side] = data["examples"]
                if data.get("errors"):
                    mobile_errors.append({"viewport":390,"source":side,"errors":data["errors"]})
    for section in local["examples"] + upstream["examples"]:
        if section["panel"] and not section.get("panelKind"):
            section["panelKind"] = "selectors" if section.get("selectors") else "properties"
    evidence = root / "interactions" / f"{slug}.json"
    interaction = read(evidence) if evidence.exists() else {"sections": [], "errors": []}
    if interaction.get("asset") != local.get("asset"):
        interaction = {"sections": [], "errors": []}
    field_operations = defaultdict(lambda: defaultdict(list))
    for section in interaction["sections"]:
        for control in section["controls"]:
            if control.get("field"):
                field_operations[section.get("anchor")][normalized(control["field"]).replace(" ", "")].append(control)
    by_title = defaultdict(list)
    for section in interaction["sections"]:
        by_title[section["title"]].append(section)
    outcomes = {}
    for section in local["examples"]:
        candidates = by_title.get(section["title"], [])
        if candidates:
            outcomes[section["index"]] = Counter(c["outcome"] for c in candidates.pop(0)["controls"])
    remaining = {e["index"]: e for e in local["examples"]}
    rows = []

    def record(reference, own, reason, note=None):
        status = "<p>" + html.escape(reason) + "</p>"
        panel_preview = None
        if reference and own and reference.get("panelKind"):
            ordinal = reference["index"] + 1
            crop = panel_captures.get((slug, ordinal))
            if crop and crop["asset"] == local.get("asset") and crop["section"] == own["anchor"]:
                stem = f"panel-preview-bounds/{slug}-{ordinal}"
                panel_preview = {"mantine": f"{stem}-mantine.png", "local": f"{stem}-local.png"}
                status += (f'<small>单独查看预览区 · <a href="{panel_preview["mantine"]}">Mantine</a> / '
                           f'<a href="{panel_preview["local"]}">Sapid</a></small>')
        mobile_images = {}
        for side, entry in (("upstream", reference), ("local", own)):
            if entry and entry["index"] < len(mobile.get(side, [])):
                narrow = mobile[side][entry["index"]]
                if narrow["anchor"] == entry["anchor"] and narrow.get("image"):
                    mobile_images[side] = f'viewport-390/{side}/{slug}/{narrow["image"]}'
        if mobile_images:
            links = " / ".join(f'<a href="{url}">{label}</a>' for side,label in (("upstream","Mantine"),("local","Sapid"))
                               if (url := mobile_images.get(side)))
            status += f'<small>390px 手机截图 · {links}</small>'
        missing_fields = []
        missing_selectors = []
        tested_fields = {}
        dedicated_fields = {}
        failed_fields = {}
        if reference and reference.get("panelKind") == "properties":
            def field_key(label):
                return normalized(label).replace(" ", "")
            labels = reference.get("fields", [])
            available = Counter(field_key(label) for label in (own or {}).get("fields", []))
            for label in labels:
                key = field_key(label)
                operations = field_operations[(own or {}).get("anchor")][key]
                tested_fields[label] = dict(Counter(control["outcome"] for control in operations))
                if available[key]:
                    available[key] -= 1
                else:
                    missing_fields.append(label)
            status += f'<small>已在本地面板定位配置名 {len(labels)-len(missing_fields)}/{len(labels)}</small>'
            if missing_fields:
                status += '<small>待核对配置：' + html.escape("、".join(missing_fields)) + '</small>'
            changed = sum(bool(outcomes.get("preview changed")) for outcomes in tested_fields.values())
            if any(tested_fields.values()):
                status += f'<small>本次构建已操作且预览发生变化的配置名 {changed}/{len(labels)}。浮层和键盘行为另见专项记录。</small>'
        if note:
            status += f"<p>{html.escape(note)}</p>"
        if reference and reference.get("panelKind") == "selectors":
            expected = {item["name"] for item in reference.get("selectors", [])}
            actual = {item["name"] for item in (own or {}).get("selectors", [])}
            missing_selectors = sorted(expected - actual)
            status += f'<small>本地 Styles API 选择器 {len(expected)-len(missing_selectors)}/{len(expected)}</small>'
            if missing_selectors:
                status += f'<small>缺少选择器：{html.escape("、".join(missing_selectors))}</small>'
            for label, outcome, file in selectors[slug]:
                status += f'<small><a href="{file}">{html.escape(label)}</a> · {html.escape(outcome)}</small>'
        if reference and reference["panel"] and reference.get("panelKind") != "selectors":
            check = (controls.get((slug, own["anchor"])) if own else None) or controls.get((slug, reference["index"] + 1)) or controls.get((slug, normalized(reference["title"])))
            if check:
                results, file = check
                for label in labels:
                    matched = [passed for name, passed in results if field_key(name) == field_key(label)]
                    if matched and not any(matched):
                        failed_fields[label] = file
                dedicated_fields = {label: file for label in labels
                                    if any(field_key(name) == field_key(label) and passed for name,passed in results)}
                good = sum(bool(v) for _, v in results)
                status += f'<p><a href="{html.escape(file, quote=True)}">观察到预览变化 {good}/{len(results)}</a></p>'
                status += "".join(f"<small>{html.escape(k)} · {'通过' if v else '需专项复核'}</small>" for k,v in results)
        if own:
            counts = outcomes.get(own["index"], {})
            status += " ".join(f"<small>{html.escape(k)} × {v}</small>" for k, v in counts.items())

        proof = actions.get((slug, own["anchor"])) if own else None
        if proof:
            status += (f'<p>浏览器实操 · <a href="{proof["before"]}">操作前</a> / '
                       f'<a href="{proof["after"]}">操作后</a></p>')
        if reference is None:
            state = "local"
        elif own is None:
            state = "missing"
        elif "部分" in reason or missing_fields or missing_selectors or reference.get("panelKind") != own.get("panelKind"):
            state = "partial"
        else:
            state = "paired"
        geometry = compare_controls(reference, own) if reference and own else []
        icons = compare_icons(reference, own) if reference and own else []
        field_icons = compare_field_icons(reference, own) if reference and own else []
        ink = None
        if (measure_images and state == "paired" and reference.get("image") and own.get("image")
                and reference["panel"] == own["panel"]):
            before = painted_bounds(root / "upstream" / slug / reference["image"])
            after = painted_bounds(root / "local" / slug / own["image"])
            if before and after:
                ink = {"mantine":before, "local":after, "deltaOffset":after["offsetX"]-before["offsetX"]}
                status += (f'<small>截图可见内容边界（含文字和线框） · {before["width"]}×{before["height"]} / '
                           f'{after["width"]}×{after["height"]} px，水平中心偏移差 {ink["deltaOffset"]:+g} px。</small>')
        for icon in icons:
            a, b = icon["mantineIcon"], icon["localIcon"]
            button_a, button_b = icon["mantineButton"], icon["localButton"]
            status += (f'<small>{html.escape(icon["name"])} · 按钮 {button_a[0]:g}×{button_a[1]:g} / '
                       f'{button_b[0]:g}×{button_b[1]:g} px；图标 {a[0]:g}×{a[1]:g} / {b[0]:g}×{b[1]:g} px；'
                       f'中心偏移 {icon["mantineOffset"]} / {icon["localOffset"]} px</small>')
        for icon in field_icons:
            a,b = icon["mantine"],icon["local"]
            status += (f'<small>输入框内图标 · {html.escape(a["label"] or b["label"])} · '
                       f'{a["side"]} / {b["side"]}；大小 {a["size"]} / {b["size"]} px；'
                       f'距边 {a["inset"]:g} / {b["inset"]:g} px；纵向中心偏移 {a["dy"]:g} / {b["dy"]:g} px</small>')
        if geometry:
            mismatches = [row for row in geometry if max(abs(row["deltaWidth"]),abs(row["deltaHeight"])) > 2]
            status += f'<small>同名控件尺寸已量测 {len(geometry)} 项，宽或高相差超过 2px 的有 {len(mismatches)} 项。</small>'
        if reference is not None:
            examples.append({"component": slug, "ordinal": reference["index"] + 1,
                             "title": reference["title"], "status": state, "note": note,
                             "panel": reference.get("panelKind"), "localPanel": (own or {}).get("panelKind"),
                             "missingFields": missing_fields, "missingSelectors": missing_selectors,
                             "fieldOperations": tested_fields, "dedicatedFieldEvidence": dedicated_fields, "failedFieldEvidence": failed_fields,
                             "asset": local.get("asset"),
                             "mantine": f"upstream/{slug}/{reference['image']}" if reference.get("image") else None,
                             "mobile": mobile_images,
                             "local": f"local/{slug}/{own['image']}" if own and own.get("image") else None,
                             "section": own["anchor"] if own else None, "geometry": geometry, "icons": icons,
                             "fieldIcons": field_icons, "action": proof, "paintedBounds": ink, "panelPreview": panel_preview})
        measured_difference = any(max(abs(item["deltaWidth"]), abs(item["deltaHeight"])) > 2 for item in geometry)
        measured_difference |= any(
            abs(a - b) > 2 for icon in icons for metric in ("Button", "Icon", "Offset")
            for a, b in zip(icon["mantine" + metric], icon["local" + metric])
        )
        for icon in field_icons:
            a,b = icon["mantine"],icon["local"]
            measured_difference |= (a["side"] != b["side"] or abs(a["inset"]-b["inset"]) > 2 or
                                    abs(a["dy"]-b["dy"]) > 2 or any(abs(x-y)>2 for x,y in zip(a["size"],b["size"])))
        observed_difference = bool(note and "仍有已确认的明显视觉或内容差异。" in note)
        rows.append(f'<tr data-observed="{int(observed_difference)}" data-difference="{int(measured_difference)}" data-state="{state}"><td>' + example_cell(reference, "upstream/" + slug) + "</td><td>" +
                    example_cell(own, "local/" + slug) + "</td><td>" + status + "</td></tr>")

    totals["操作后预览发生变化"] += sum(row.get("preview changed", 0) for row in outcomes.values())
    for ref in upstream["examples"]:
        key = slug, ref["index"] + 1
        anchor, state, note = mapping.get(key, (None, None, None))
        if key not in mapping and ref["index"] < len(baseline):
            saved = baseline[ref["index"]]
            if (saved["title"] == ref["title"] and saved.get("panel") == ref.get("panelKind")
                    and saved.get("section") and any(e["anchor"] == saved["section"] for e in local["examples"])):
                anchor, state = saved["section"], saved["status"]
        exact = [e for e in remaining.values() if normalized(e["title"]) == normalized(ref["title"])
                 and e.get("panelKind") == ref.get("panelKind")]
        explicit = next((e for e in local["examples"] if e["anchor"] == anchor), None)
        if explicit and explicit.get("panelKind") != ref.get("panelKind"):
            if exact:
                explicit, state, note = exact[0], "paired", None
        missing = state and any(word in state for word in ("missing", "absent", "unpaired"))
        own = explicit or (exact[0] if exact and not missing else None)
        reason = "已定位对应章节" if explicit else "同标题截图"
        if state and ("partial" in state or "grouped" in state) and explicit:
            reason = "组合示例，部分覆盖"
        if own is None:
            reason = "尚缺本地对应示例" if missing else "尚未定位本地对应示例"
        if own:
            remaining.pop(own["index"], None)
        if own and ref["panel"] and not own["panel"]:
            reason += "，截图中未检测到本地侧栏"
        elif own and ref.get("panelKind") != own.get("panelKind"):
            reason += "，两侧面板类型不同"
        record(ref, own, reason, note)
    for own in remaining.values():
        record(None, own, "本地独立示例或不同标题")
    n_ref = len(upstream["examples"])
    n_local = len(local["examples"])
    totals["官方内嵌示例"] += n_ref
    totals["本地内嵌示例"] += n_local
    totals["官方截图"] += sum(bool(e.get("image")) for e in upstream["examples"])
    totals["本地截图"] += sum(bool(e.get("image")) for e in local["examples"])
    problems = upstream["errors"] + local["errors"] + interaction["errors"] + mobile_errors
    errors = f"<p>{html.escape(json.dumps(problems, ensure_ascii=False))}</p>" if problems else ""
    return (f'<details id="{slug}"><summary>{html.escape(local["name"])} '
            f'<small>{n_ref} / {n_local}</small></summary><p>'
            f'<a href="{html.escape(upstream["url"], quote=True)}">官方页面</a> · '
            f'<a href="{html.escape(local["url"], quote=True)}">本地页面</a> · '
            f'<a href="interactions/{slug}.json">逐项操作记录</a></p>{errors}'
            '<table><thead><tr><th>Mantine</th><th>Sapid</th><th>核对记录</th></tr></thead><tbody>'
            + "".join(rows) + "</tbody></table></details>")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("directory", type=Path, help="example-review directory under the web Cargo target")
    parser.add_argument("--measure-images", action="store_true", help="measure foreground geometry in each paired screenshot using Pillow")
    parser.add_argument("--check-panels", action="store_true", help="fail on missing panels, missing selectors, or properties without a verified operation")
    parser.add_argument("--write-crosswalk", metavar="MANTINE_VERSION", help="save the official example order and locally verified section IDs in docs/")
    args = parser.parse_args()
    root, measure_images = args.directory, args.measure_images
    totals = Counter()
    mapping = pairings(root)
    controls = verified_controls(root)
    selectors = selector_evidence(root)
    actions = action_proofs(root)
    capture_file = root / "panel-preview-bounds/capture.json"
    panel_captures = {(row["slug"], row["ordinal"]): row for row in read(capture_file).get("captures", [])} if capture_file.is_file() else {}
    baseline = read(CROSSWALK)["components"] if CROSSWALK.exists() else {}
    examples = []
    slugs = sorted(file.parent.name for file in (root / "local").glob("*/index.json"))
    pages = [component(root, slug, totals, mapping, controls, selectors, actions, examples, measure_images, panel_captures,
                       baseline.get(slug, [])) for slug in slugs]
    state_counts = Counter(row["status"] for row in examples)
    totals.update({"已配对官方示例": state_counts["paired"], "部分覆盖": state_counts["partial"],
                   "缺少对应示例": state_counts["missing"]})
    stylesheet = """
body{font:15px/1.5 system-ui,sans-serif;color:#20262a;max-width:1280px;margin:auto;padding:24px}
a{color:#176bb2}summary{cursor:pointer;padding:8px 0;font-size:17px;font-weight:600}
details{border-top:1px solid #dce2e6}table{width:100%;border-collapse:collapse;table-layout:fixed}
th,td{text-align:left;vertical-align:top;padding:10px;border:1px solid #dce2e6}
th:last-child{width:18%}img{display:block;max-width:100%;max-height:290px;margin:8px auto}
form{display:flex;align-items:center;gap:16px;flex-wrap:wrap;margin:16px 0}input,select{padding:5px;font:inherit}small{display:block;color:#52616a}summary small{display:inline;margin-left:12px}
"""
    stats = " · ".join(f"{k} {v:,}" for k, v in totals.items())
    filters = """<form id="filters" onsubmit="return false"><label>组件 <input id="component" type="search" placeholder="NativeSelect"></label>
<label>示例 <select id="status"><option value="">全部</option><option value="missing">缺少对应示例</option><option value="partial">部分覆盖</option><option value="observed">截图中已注明明显差异</option><option value="geometry">已量测尺寸或图标偏差超过 2 px</option><option value="paired">已有配对</option><option value="local">本地其他示例</option></select></label>
<a href="examples.json">下载逐例记录</a> · <a href="configurations.json">下载逐配置操作记录</a></form>"""
    filtering = """<script>
const search = document.querySelector('#component'), statusFilter = document.querySelector('#status');
function filter() {
  const term = search.value.toLowerCase().trim(), status = statusFilter.value;
  document.querySelectorAll('details').forEach(section => {
    const matches = section.id.includes(term) || section.querySelector('summary').textContent.toLowerCase().includes(term);
    let count = 0;
    for (const row of section.querySelectorAll('tr[data-state]')) {
      row.hidden = status === 'observed' ? row.dataset.observed !== '1' : status === 'geometry' ? row.dataset.difference !== '1' : !!status && row.dataset.state !== status;
      if (!row.hidden) count++;
    }
    section.hidden = !matches || !count;
    if (term) section.open = !section.hidden;
  });
}
search.addEventListener('input', filter); statusFilter.addEventListener('change', filter);
const query = new URLSearchParams(location.search);
if (query.has('component')) search.value = query.get('component');
if ([...statusFilter.options].some(option => option.value === query.get('status'))) statusFilter.value = query.get('status');
if (search.value || statusFilter.value) filter();
const initial = document.getElementById(decodeURIComponent(location.hash.slice(1)));
if (initial?.tagName === 'DETAILS') { initial.open = true; initial.scrollIntoView(); }
</script>"""
    evidence_links = [
        ("responsive-native-select/comparison.html", "NativeSelect 五种宽度的配置与箭头"),
        ("center-final/review.html", "水平位置修复前后与手机截图"),
        ("display-latest/review.html", "组件桌面与手机逐例核对"),
    ]
    evidence_navigation = " · ".join(f'<a href="{url}">{label}</a>' for url,label in evidence_links if (root/url).is_file())
    document = ('<!doctype html><html lang="zh-CN"><meta charset="UTF-8"><meta name="viewport" '
                'content="width=device-width,initial-scale=1"><title>Mantine 与 Sapid 逐例核对</title>'
                f'<style>{stylesheet}</style><h1>Mantine 与 Sapid 逐例核对</h1><p>{len(pages)} 页 · {stats}</p>'
                '<p>每页列出所有已捕获的示例。已定位章节以及同标题示例并排显示。点击图片可查看原始尺寸。'
                '不同标题保留在清单中，便于检查一个本地示例展示多个官方示例的情况。'
                '操作记录中的 needs contextual check 表示需要打开浮层、悬停、滚动或进一步检查。'
                '已定位的章节仍可能存在文案、尺寸或间距差异，具体观察列在截图旁。</p>'
                + f'<p>{evidence_navigation}</p>' + filters + "".join(pages) + filtering + '</html>')
    (root / "examples.json").write_text(json.dumps(examples, ensure_ascii=False, indent=1), encoding="utf-8")
    configurations = [
        {"component": row["component"], "section": row["section"], "field": name, "asset": row["asset"],
         "operations": operations, "dedicatedCheck": row["dedicatedFieldEvidence"].get(name),
         "failedCheck": row["failedFieldEvidence"].get(name)}
        for row in examples for name,operations in row["fieldOperations"].items()
    ]
    (root / "configurations.json").write_text(json.dumps(configurations, ensure_ascii=False, indent=1), encoding="utf-8")
    (root / "index.html").write_text(document, encoding="utf-8")
    print(json.dumps(dict(totals), ensure_ascii=True))
    if args.check_panels:
        gaps = []
        for row in examples:
            if not row["panel"]:
                continue
            if row["panel"] != row["localPanel"]:
                gaps.append((row["component"], row["ordinal"], "panel", row["panel"]))
            for name in row["missingFields"] + row["missingSelectors"]:
                gaps.append((row["component"], row["ordinal"], "absent", name))
            for name, file in row["failedFieldEvidence"].items():
                gaps.append((row["component"], row["ordinal"], "failed", name, file))
            for name, operations in row["fieldOperations"].items():
                if (name not in row["failedFieldEvidence"] and not operations.get("preview changed")
                        and not row["dedicatedFieldEvidence"].get(name)):
                    gaps.append((row["component"], row["ordinal"], "unverified", name))
        if gaps or not any(row["panel"] for row in examples):
            raise SystemExit("Panel verification failed: " + json.dumps(gaps, ensure_ascii=True))
    if args.write_crosswalk:
        if not examples:
            raise SystemExit("No examples were captured")
        components = {slug: [] for slug in slugs}
        upstream = {}
        for row in examples:
            entry = {"title": row["title"], "status": row["status"]}
            if row["section"]:
                entry["section"] = row["section"]
            if row["panel"]:
                entry["panel"] = row["panel"]
                if row["panel"] == "properties":
                    entry["fields"] = list(row["fieldOperations"])
                else:
                    slug = row["component"]
                    if slug not in upstream:
                        upstream[slug] = read(root / "upstream" / slug / "index.json")["examples"]
                    entry["selectors"] = [s["name"] for s in upstream[slug][row["ordinal"] - 1]["selectors"]]
            components[row["component"]].append(entry)
        data = {"mantineVersion": args.write_crosswalk, "components": components}
        CROSSWALK.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
