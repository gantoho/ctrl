use dioxus::prelude::*;
use ctrl::prelude::*;
use std::rc::Rc;

// ── Drawer 演示 ──
#[component]
#[allow(non_snake_case)]
pub fn DrawerDocs() -> Element {
    let mut visible = use_signal(|| false);
    rsx! {
        div {
            Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| visible.set(true), "打开抽屉" }
            Drawer {
                visible: visible(),
                title: "抽屉标题".to_string(),
                onclose: move |_| visible.set(false),
                p { "抽屉内容区域，可以放表单、说明等任意内容。" }
            }
        }
    }
}

// ── Notification 演示 ──
#[component]
#[allow(non_snake_case)]
pub fn NotificationDocs() -> Element {
    let mut api = use_notification();
    rsx! {
        Space { wrap: true, gap: "sm".to_string(),
            Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| api.info("信息通知".to_string(), "这是一条普通信息通知。".to_string()), "信息" }
            Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| api.success("操作成功".to_string(), "数据已成功保存。".to_string()), "成功" }
            Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| api.warning("警告".to_string(), "存储空间不足，请及时清理。".to_string()), "警告" }
            Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| api.error("错误".to_string(), "网络请求失败，请重试。".to_string()), "错误" }
        }
    }
}

// ── Dropdown 演示 ──
#[component]
#[allow(non_snake_case)]
pub fn DropdownDocs() -> Element {
    let mut selected = use_signal(|| String::new());
    rsx! {
        div {
            Dropdown {
                trigger: rsx! { Button { variant: Variant::Primary, size: Size::Sm, "打开菜单" } },
                DropdownItem {
                    onclick: move |_| selected.set("选项一".to_string()),
                    "选项一"
                }
                DropdownItem {
                    onclick: move |_| selected.set("选项二".to_string()),
                    "选项二"
                }
                DropdownDivider {}
                DropdownItem { disabled: true, "禁用项" }
            }
            if !selected().is_empty() {
                p { "已选：{selected}" }
            }
        }
    }
}

// ── Menu 演示 ──
#[component]
#[allow(non_snake_case)]
pub fn MenuDocs() -> Element {
    rsx! {
        Menu {
            MenuItem { "首页" }
            MenuItem { "组件" }
            MenuItem { "文档" }
            MenuItem { disabled: true, "禁用项" }
        }
    }
}

// ── Button 交互演示 ──

#[component]
#[allow(non_snake_case)]
pub fn ButtonInteract() -> Element {
    let mut count = use_signal(|| 0);
    let mut disabled = use_signal(|| false);

    rsx! {
        Space { wrap: true, gap: "sm".to_string(),
            Button {
                variant: Variant::Primary,
                disabled: disabled(),
                onclick: move |_| count.set(count() + 1),
                "点击次数: {count()}"
            }
            Button {
                variant: Variant::Ghost,
                onclick: move |_| disabled.set(!disabled()),
                if disabled() { "恢复" } else { "禁用" }
            }
            Button {
                variant: Variant::Outline,
                onclick: move |_| count.set(0),
                "重置"
            }
        }
    }
}

// ── Progress 动态演示 ──

#[component]
#[allow(non_snake_case)]
pub fn ProgressDynamicDemo() -> Element {
    let mut percent = use_signal(|| 30.0);
    rsx! {
        Space { direction: Direction::Vertical, gap: "sm".to_string(),
            Progress { percent: percent(), color: "var(--ctrl-primary)".to_string(), show_text: true }
            Space { gap: "xs".to_string(),
                Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| { let p = percent(); if p > 0.0 { percent.set((p - 10.0).max(0.0)); } }, "-10" }
                Button { variant: Variant::Outline, size: Size::Sm, onclick: move |_| { let p = percent(); if p < 100.0 { percent.set((p + 10.0).min(100.0)); } }, "+10" }
            }
        }
    }
}

// ── Tabs 基本演示 ──

#[component]
#[allow(non_snake_case)]
pub fn TabsBasicDemo() -> Element {
    let mut active = use_signal(|| "tab1".to_string());
    let items = vec![
        ("tab1".to_string(), "标签一".to_string(), false),
        ("tab2".to_string(), "标签二".to_string(), false),
        ("tab3".to_string(), "标签三".to_string(), false),
    ];
    rsx! {
        div { style: "width: 100%; max-width: 480px;",
            TabNav { items: items, active: active(), onchange: move |v| active.set(v) }
            TabContent {
                div { style: "padding: 16px 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-md);",
                    if active() == "tab1" { "这是标签一的内容区域。" }
                    else if active() == "tab2" { "这是标签二的内容区域。" }
                    else { "这是标签三的内容区域。" }
                }
            }
        }
    }
}

// ── Tabs 禁用演示 ──

#[component]
#[allow(non_snake_case)]
pub fn TabsDisabledDemo() -> Element {
    let mut active = use_signal(|| "d1".to_string());
    let items = vec![
        ("d1".to_string(), "可用".to_string(), false),
        ("d2".to_string(), "禁用".to_string(), true),
        ("d3".to_string(), "可用".to_string(), false),
    ];
    rsx! {
        div { style: "width: 100%; max-width: 480px;",
            TabNav { items: items, active: active(), onchange: move |v| active.set(v) }
            TabContent {
                div { style: "padding: 16px 0; color: var(--ctrl-text-secondary); font-size: var(--ctrl-font-size-md);",
                    "当前选中: {active()}"
                }
            }
        }
    }
}

// ── Pagination 演示 ──

#[component]
#[allow(non_snake_case)]
pub fn PaginationDocsDemo() -> Element {
    let mut page = use_signal(|| 1u32);
    rsx! {
        Space { direction: Direction::Vertical, gap: "sm".to_string(),
            Pagination { current: page(), total: 50, page_size: 10, onchange: move |v| page.set(v) }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "当前第 {page()} 页" }
        }
    }
}

// ── Alert 全局横幅演示 ──

#[component]
#[allow(non_snake_case)]
pub fn AlertBannerDocsDemo() -> Element {
    const MAX_ALERTS: usize = 5;

    let mut alerts = use_signal(|| Vec::<(u32, bool)>::new()); // (id, closing)
    let mut next_id = use_signal(|| 0u32);

    let mut add_alert = move || {
        let id = next_id();
        next_id.set(id + 1);

        let mut list = alerts.write();
        let active_count = list.iter().filter(|(_, cl)| !cl).count();
        if active_count >= MAX_ALERTS {
            if let Some(oldest) = list.iter_mut().find(|(_, cl)| !*cl) {
                oldest.1 = true;
            }
        }
        list.push((id, false));
    };

    rsx! {
        AlertBannerContainer {
            for (id, closing) in alerts().iter() {
                {
                    let alert_id = *id;
                    let cl = *closing;
                    rsx! {
                        Alert {
                            key: "{alert_id}",
                            r#type: AlertType::Warning,
                            title: "存储空间不足".to_string(),
                            description: "您的存储空间已使用 95%，请尽快清理文件。".to_string(),
                            mode: AlertMode::Banner,
                            closable: true,
                            closing: cl,
                            duration: 5000,
                            onclose: move |_| alerts.write().retain(|(aid, _)| *aid != alert_id),
                        }
                    }
                }
            }
        }
        div {
            Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| add_alert(), "显示全局横幅" }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); margin-left: 12px;",
                "最多同时显示 5 条，超出后从最旧的开始依次退出"
            }
        }
    }
}

// ── Message 点击触发演示 ──

#[component]
#[allow(non_snake_case)]
pub fn MessageTriggerDocsDemo() -> Element {
    const MAX_MESSAGES: usize = 5;

    type MsgItem = (u32, MessageType, String, bool); // id, type, content, closing

    let mut messages = use_signal(|| Vec::<MsgItem>::new());
    let next_id = use_signal(|| 0u32);

    rsx! {
        MessageContainer {
            for (id, m_type, content, closing) in messages().iter() {
                {
                    let msg_id = *id;
                    let t = m_type.clone();
                    let c2 = content.clone();
                    let cl = *closing;
                    rsx! {
                        Message {
                            key: "{msg_id}",
                            r#type: t,
                            content: c2,
                            closing: cl,
                            duration: 3000,
                            onclose: move |_| messages.write().retain(|(mid, _, _, _)| *mid != msg_id),
                        }
                    }
                }
            }
        }
        Space { direction: Direction::Vertical, gap: "sm".to_string(),
            Space { wrap: true, gap: "xs".to_string(),
                Button { variant: Variant::Outline, size: Size::Sm, onclick: {
                    let mut nid = next_id; let mut msg = messages;
                    move |_| {
                        let id = nid();
                        nid.set(id + 1);
                        let mut list = msg.cloned();
                        if list.iter().filter(|(_,_,_,cl)| !cl).count() >= MAX_MESSAGES {
                            if let Some(o) = list.iter_mut().find(|(_,_,_,cl)| !*cl) { o.3 = true; }
                        }
                        list.push((id, MessageType::Info, "已复制到剪贴板".into(), false));
                        msg.set(list);
                    }
                }, "Info" }
                Button { variant: Variant::Outline, size: Size::Sm, onclick: {
                    let mut nid = next_id; let mut msg = messages;
                    move |_| {
                        let id = nid();
                        nid.set(id + 1);
                        let mut list = msg.cloned();
                        if list.iter().filter(|(_,_,_,cl)| !cl).count() >= MAX_MESSAGES {
                            if let Some(o) = list.iter_mut().find(|(_,_,_,cl)| !*cl) { o.3 = true; }
                        }
                        list.push((id, MessageType::Success, "保存成功！".into(), false));
                        msg.set(list);
                    }
                }, "Success" }
                Button { variant: Variant::Outline, size: Size::Sm, onclick: {
                    let mut nid = next_id; let mut msg = messages;
                    move |_| {
                        let id = nid();
                        nid.set(id + 1);
                        let mut list = msg.cloned();
                        if list.iter().filter(|(_,_,_,cl)| !cl).count() >= MAX_MESSAGES {
                            if let Some(o) = list.iter_mut().find(|(_,_,_,cl)| !*cl) { o.3 = true; }
                        }
                        list.push((id, MessageType::Warning, "文件格式不支持".into(), false));
                        msg.set(list);
                    }
                }, "Warning" }
                Button { variant: Variant::Outline, size: Size::Sm, onclick: {
                    let mut nid = next_id; let mut msg = messages;
                    move |_| {
                        let id = nid();
                        nid.set(id + 1);
                        let mut list = msg.cloned();
                        if list.iter().filter(|(_,_,_,cl)| !cl).count() >= MAX_MESSAGES {
                            if let Some(o) = list.iter_mut().find(|(_,_,_,cl)| !*cl) { o.3 = true; }
                        }
                        list.push((id, MessageType::Error, "网络连接超时".into(), false));
                        msg.set(list);
                    }
                }, "Error" }
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "当前消息数量: {messages().len()}（上限 {MAX_MESSAGES}）" }
        }
    }
}

// ── Message 位置演示 ──

#[component]
#[allow(non_snake_case)]
pub fn MessagePositionDocsDemo() -> Element {
    const MAX_MESSAGES: usize = 5;

    let mut messages = use_signal(|| Vec::<(u32, bool)>::new()); // (id, closing)
    let mut next_id = use_signal(|| 0u32);
    let mut pos_placement = use_signal(|| MessagePlacement::Top);

    let mut add_message = move || {
        let id = next_id();
        next_id.set(id + 1);

        let mut list = messages.write();
        let active_count = list.iter().filter(|(_, cl)| !cl).count();
        if active_count >= MAX_MESSAGES {
            if let Some(oldest) = list.iter_mut().find(|(_, cl)| !*cl) {
                oldest.1 = true;
            }
        }
        list.push((id, false));
    };

    rsx! {
        MessageContainer {
            placement: pos_placement(),
            for (id, closing) in messages().iter() {
                {
                    let msg_id = *id;
                    let cl = *closing;
                    rsx! {
                        Message {
                            key: "{msg_id}",
                            r#type: MessageType::Success,
                            content: "消息已发送".to_string(),
                            closing: cl,
                            duration: 3000,
                            onclose: move |_| messages.write().retain(|(mid, _)| *mid != msg_id),
                        }
                    }
                }
            }
        }
        Space { direction: Direction::Vertical, gap: "sm".to_string(),
            Space { wrap: true, gap: "xs".to_string(),
                Button { variant: if pos_placement() == MessagePlacement::Top { Variant::Primary } else { Variant::Ghost }, size: Size::Sm, onclick: move |_| pos_placement.set(MessagePlacement::Top), "Top" }
                Button { variant: if pos_placement() == MessagePlacement::TopRight { Variant::Primary } else { Variant::Ghost }, size: Size::Sm, onclick: move |_| pos_placement.set(MessagePlacement::TopRight), "TopRight" }
                Button { variant: if pos_placement() == MessagePlacement::TopLeft { Variant::Primary } else { Variant::Ghost }, size: Size::Sm, onclick: move |_| pos_placement.set(MessagePlacement::TopLeft), "TopLeft" }
                Button { variant: if pos_placement() == MessagePlacement::Bottom { Variant::Primary } else { Variant::Ghost }, size: Size::Sm, onclick: move |_| pos_placement.set(MessagePlacement::Bottom), "Bottom" }
            }
            Button { variant: Variant::Primary, size: Size::Sm, onclick: move |_| add_message(), "发送消息" }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);", "当前消息数量: {messages().len()}（上限 {MAX_MESSAGES}）" }
        }
    }
}

// ── Input 演示 ──

#[component]
#[allow(non_snake_case)]
pub fn BasicInputDemo() -> Element {
    let mut value = use_signal(|| String::new());

    rsx! {
        Space { direction: Direction::Vertical, gap: "sm".to_string(),
            Input {
                placeholder: "请输入内容",
                value: value(),
                oninput: move |v: String| value.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前输入: "
                em { style: "font-style: normal; color: var(--ctrl-primary); font-weight: 500;", "{value()}" }
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn FormValidationDemo() -> Element {
    let mut username = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut confirm = use_signal(|| String::new());
    let mut submitted = use_signal(|| false);
    let mut errors = use_signal(|| Vec::new());

    rsx! {
        div { style: "max-width: 400px;",
            Form {
                onsubmit: move |_data: Rc<FormData>| {
                    // 手动校验
                    let mut errs = Vec::new();
                    if username().trim().is_empty() { errs.push("用户名不能为空".to_string()); }
                    if email().trim().is_empty() { errs.push("邮箱不能为空".to_string()); }
                    if password().trim().is_empty() { errs.push("密码不能为空".to_string()); }
                    if password() != confirm() { errs.push("两次密码输入不一致".to_string()); }
                    if errs.is_empty() {
                        submitted.set(true);
                        errors.set(Vec::new());
                    } else {
                        errors.set(errs);
                    }
                },
                FormItem {
                    label: "用户名".to_string(),
                    required: true,
                    name: "username".to_string(),
                    value: username(),
                    Input {
                        placeholder: "请输入用户名",
                        value: username(),
                        oninput: move |v: String| { username.set(v); submitted.set(false); },
                    }
                }
                FormItem {
                    label: "邮箱".to_string(),
                    required: true,
                    name: "email".to_string(),
                    value: email(),
                    Input {
                        placeholder: "请输入邮箱",
                        value: email(),
                        oninput: move |v: String| { email.set(v); submitted.set(false); },
                    }
                }
                FormItem {
                    label: "密码".to_string(),
                    required: true,
                    name: "password".to_string(),
                    value: password(),
                    Input {
                        r#type: "password",
                        placeholder: "请输入密码",
                        value: password(),
                        oninput: move |v: String| { password.set(v); submitted.set(false); },
                    }
                }
                FormItem {
                    label: "确认密码".to_string(),
                    required: true,
                    name: "confirm".to_string(),
                    value: confirm(),
                    Input {
                        r#type: "password",
                        placeholder: "请再次输入密码",
                        value: confirm(),
                        oninput: move |v: String| { confirm.set(v); submitted.set(false); },
                    }
                }
                if !errors().is_empty() {
                    div {
                        style: "margin-bottom: 12px; padding: 8px 12px; background: #fef2f2; border: 1px solid #fecaca; border-radius: 6px; font-size: 13px; color: #dc2626;",
                        for err in errors() {
                            div { key: "{err}", "{err}" }
                        }
                    }
                }
                FormItem {
                    Button { variant: Variant::Primary, block: true, r#type: "submit", "注册" }
                }
            }
            if submitted() {
                div {
                    style: "margin-top: 16px; padding: 12px; background: var(--ctrl-primary-light, #e8f4fd); border-radius: 6px; font-size: 14px; color: var(--ctrl-primary, #1a73e8);",
                    "提交成功！用户名: {username()}，邮箱: {email()}"
                }
            }
        }
    }
}


// ── Switch 演示 ──

#[component]
#[allow(non_snake_case)]
pub fn BasicSwitchDemo() -> Element {
    let mut on = use_signal(|| false);

    rsx! {
        Space { gap: "md".to_string(),
            Switch {
                checked: on(),
                onchange: move |v| on.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-md); color: var(--ctrl-text);",
                if on() { "已开启" } else { "已关闭" }
            }
        }
    }
}

// ── Checkbox 演示 ──

#[component]
#[allow(non_snake_case)]
pub fn BasicCheckboxDemo() -> Element {
    let mut checked = use_signal(|| false);

    rsx! {
        Space { direction: Direction::Vertical, gap: "xs".to_string(),
            Checkbox {
                checked: checked(),
                label: "同意使用协议".to_string(),
                onchange: move |v| checked.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前状态: "
                em { style: "font-style: normal; color: var(--ctrl-primary); font-weight: 500;",
                    if checked() { "已同意" } else { "未同意" }
                }
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn CheckAllDemo() -> Element {
    let items: Vec<&'static str> = vec!["选项 A", "选项 B", "选项 C"];
    let items_len = items.len();
    let mut checked = use_signal(|| vec![false; items_len]);

    let all = checked().iter().all(|&c| c);
    let some = checked().iter().any(|&c| c);
    let indet = some && !all;

    rsx! {
        Space { direction: Direction::Vertical, gap: "xs".to_string(),
            Checkbox {
                checked: all,
                indeterminate: indet,
                label: "全选".to_string(),
                onchange: move |v| checked.set(vec![v; items_len]),
            }
            div { style: "height: 1px; background: var(--ctrl-border); margin: 4px 0; width: 100%;" }
            {items.iter().enumerate().map(|(i, item)| {
                let idx = i;
                let label = format!("{}", item);
                rsx! {
                    Checkbox {
                        key: "{idx}",
                        checked: checked()[idx],
                        label: label,
                        onchange: move |v| {
                            let mut c = checked();
                            c[idx] = v;
                            checked.set(c);
                        },
                    }
                }
            })}
        }
    }
}

// ── Radio 演示 ──

#[component]
#[allow(non_snake_case)]
pub fn BasicRadioDemo() -> Element {
    let mut selected = use_signal(|| "a".to_string());

    rsx! {
        Space { direction: Direction::Vertical, gap: "xs".to_string(),
            Radio {
                value: "a".to_string(),
                label: "选项 A".to_string(),
                checked: selected() == "a",
                onchange: move |v| selected.set(v),
            }
            Radio {
                value: "b".to_string(),
                label: "选项 B".to_string(),
                checked: selected() == "b",
                onchange: move |v| selected.set(v),
            }
            Radio {
                value: "c".to_string(),
                label: "选项 C（禁用）".to_string(),
                checked: selected() == "c",
                disabled: true,
                onchange: move |_| {},
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary); margin-top: 4px;",
                "当前选中: "
                em { style: "font-style: normal; color: var(--ctrl-primary); font-weight: 500;", "{selected()}" }
            }
        }
    }
}

// ── Select 演示 ──

#[component]
#[allow(non_snake_case)]
pub fn BasicSelectDemo() -> Element {
    let mut value = use_signal(|| String::new());

    let options = vec![
        ("a".to_string(), "选项 A".to_string(), false),
        ("b".to_string(), "选项 B".to_string(), false),
        ("c".to_string(), "选项 C".to_string(), true),
    ];

    rsx! {
        Space { direction: Direction::Vertical, gap: "sm".to_string(),
            Select {
                options: options,
                placeholder: "请选择".to_string(),
                value: value(),
                onchange: move |v| value.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前选中: "
                em { style: "font-style: normal; color: var(--ctrl-primary); font-weight: 500;",
                    if value().is_empty() { "无" } else { "{value()}" }
                }
            }
        }
    }
}

// ── Dialog 演示 ──

#[component]
#[allow(non_snake_case)]
pub fn BasicDialogDemo() -> Element {
    let mut visible = use_signal(|| false);

    rsx! {
        div {
            Button { variant: Variant::Primary, onclick: move |_| visible.set(true), "打开对话框" }
            Dialog {
                visible: visible(),
                title: "提示".to_string(),
                onclose: move |_| visible.set(false),
                p { "这是一条提示信息。对话框通过 visible 控制显示/隐藏，点击遮罩或右上角关闭按钮均可关闭。" }
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn FooterDialogDemo() -> Element {
    let mut visible = use_signal(|| false);

    rsx! {
        div {
            Button { variant: Variant::Outline, onclick: move |_| visible.set(true), "确认对话框" }
            Dialog {
                visible: visible(),
                title: "确认操作".to_string(),
                onclose: move |_| visible.set(false),
                footer: rsx! {
                    Button { variant: Variant::Ghost, onclick: move |_| visible.set(false), "取消" }
                    Button {
                        variant: Variant::Primary,
                        onclick: move |_| visible.set(false),
                        "确定"
                    }
                },
                Space { direction: Direction::Vertical, gap: "xs".to_string(),
                    p { "确定要执行此操作吗？" }
                    p { "此操作不可撤销，请谨慎操作。" }
                }
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn DialogImperativeDemo() -> Element {
    let mut visible = use_signal(|| false);

    rsx! {
        Button {
            variant: Variant::Primary,
            onclick: move |_| visible.set(true),
            "命令式打开对话框"
        }
        Dialog {
            visible: visible(),
            title: "确认删除".to_string(),
            onclose: move |_| visible.set(false),
            p { "删除后不可恢复，确定继续？" }
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn DrawerImperativeDemo() -> Element {
    let mut visible = use_signal(|| false);

    rsx! {
        Button {
            variant: Variant::Primary,
            onclick: move |_| visible.set(true),
            "命令式打开抽屉"
        }
        Drawer {
            visible: visible(),
            title: "用户详情".to_string(),
            onclose: move |_| visible.set(false),
            p { "这里是抽屉的内容区域。" }
        }
    }
}

// ── Slider 演示 ──

#[component]
#[allow(non_snake_case)]
pub fn BasicSliderDemo() -> Element {
    let mut value = use_signal(|| 50);

    rsx! {
        div { style: "max-width: 400px;",
            Slider {
                value: value(),
                onchange: move |v| value.set(v),
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn CustomRangeSliderDemo() -> Element {
    let mut v1 = use_signal(|| 30);
    let mut v2 = use_signal(|| 0);

    rsx! {
        Space { direction: Direction::Vertical, gap: "lg".to_string(),
            Slider {
                value: v1(),
                min: 0,
                max: 100,
                step: 5,
                show_label: true,
                onchange: move |v| v1.set(v),
            }
            Slider {
                value: v2(),
                min: -50,
                max: 50,
                step: 10,
                show_label: true,
                onchange: move |v| v2.set(v),
            }
        }
    }
}

// ── Rate 演示 ──

#[component]
#[allow(non_snake_case)]
pub fn BasicRateDemo() -> Element {
    let mut value = use_signal(|| 3.0);

    rsx! {
        Space { direction: Direction::Vertical, gap: "xs".to_string(),
            Rate {
                value: value(),
                onchange: move |v: f64| value.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前评分: {value()}"
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn HalfRateDemo() -> Element {
    let mut value = use_signal(|| 2.5);

    rsx! {
        Space { direction: Direction::Vertical, gap: "xs".to_string(),
            Rate {
                value: value(),
                allow_half: true,
                onchange: move |v: f64| value.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前评分: {value()}"
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn CustomIconRateDemo() -> Element {
    let mut value = use_signal(|| 3.0);

    // 用 SVG data URI 模拟自定义图标
    let icon_full = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'%3E%3Ccircle cx='10' cy='10' r='8' fill='%23f5a623'/%3E%3C/svg%3E";
    let icon_empty = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'%3E%3Ccircle cx='10' cy='10' r='7' fill='none' stroke='%23ccc' stroke-width='2'/%3E%3C/svg%3E";

    rsx! {
        Space { direction: Direction::Vertical, gap: "xs".to_string(),
            Rate {
                value: value(),
                icon_full: icon_full.to_string(),
                icon_empty: icon_empty.to_string(),
                onchange: move |v: f64| value.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前评分: {value()}"
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn CustomIconHalfRateDemo() -> Element {
    let mut value = use_signal(|| 2.5);

    let icon_full = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'%3E%3Ccircle cx='10' cy='10' r='8' fill='%23f5a623'/%3E%3C/svg%3E";
    let icon_half = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'%3E%3Cdefs%3E%3ClinearGradient id='g'%3E%3Cstop offset='50%25' stop-color='%23f5a623'/%3E%3Cstop offset='50%25' stop-color='%23ccc'/%3E%3C/linearGradient%3E%3C/defs%3E%3Ccircle cx='10' cy='10' r='7' fill='url(%23g)' stroke='%23ccc' stroke-width='1'/%3E%3C/svg%3E";
    let icon_empty = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'%3E%3Ccircle cx='10' cy='10' r='7' fill='none' stroke='%23ccc' stroke-width='2'/%3E%3C/svg%3E";

    rsx! {
        Space { direction: Direction::Vertical, gap: "xs".to_string(),
            Rate {
                value: value(),
                allow_half: true,
                icon_full: icon_full.to_string(),
                icon_half: icon_half.to_string(),
                icon_empty: icon_empty.to_string(),
                onchange: move |v: f64| value.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前评分: {value()}"
            }
        }
    }
}

// ── Segmented 演示 ──

#[component]
#[allow(non_snake_case)]
pub fn BasicSegmentedDemo() -> Element {
    let mut value = use_signal(|| String::new());

    let options = vec![("日".to_string(), "day".to_string()), ("周".to_string(), "week".to_string()), ("月".to_string(), "month".to_string()), ("年".to_string(), "year".to_string())];

    rsx! {
        Space { direction: Direction::Vertical, gap: "sm".to_string(),
            Segmented {
                options: options,
                value: value(),
                onchange: move |v| value.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前选中: "
                em { style: "font-style: normal; color: var(--ctrl-primary); font-weight: 500;",
                    if value().is_empty() { "无" } else { "{value()}" }
                }
            }
        }
    }
}

// ── InputNumber 演示 ──

#[component]
#[allow(non_snake_case)]
pub fn BasicInputNumberDemo() -> Element {
    let mut value = use_signal(|| 0);

    rsx! {
        Space { direction: Direction::Vertical, gap: "sm".to_string(),
            InputNumber {
                value: value(),
                onchange: move |v| value.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前值: {value()}"
            }
        }
    }
}

// ── Upload 演示 ──

#[component]
#[allow(non_snake_case)]
pub fn ClickUploadDemo() -> Element {
    let mut files = use_signal(|| Vec::new());

    rsx! {
        Upload {
            files: files(),
            onchange: move |f| files.set(f),
            Button { variant: Variant::Primary, "选择文件" }
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn FileListUploadDemo() -> Element {
    let mut files = use_signal(|| vec![
        UploadFile { name: "photo.jpg".to_string(), size: 204800, status: UploadFileStatus::Success },
        UploadFile { name: "document.pdf".to_string(), size: 1024000, status: UploadFileStatus::Success },
    ]);

    rsx! {
        Upload {
            files: files(),
            onremove: move |i| {
                let mut f = files();
                f.remove(i);
                files.set(f);
            },
            Button { variant: Variant::Primary, "选择文件" }
        }
    }
}

// ── Form 演示 ──

// ── Upload 拖拽上传演示 ──

#[component]
#[allow(non_snake_case)]
pub fn DragUploadDemo() -> Element {
    let mut files = use_signal(|| Vec::new());

    rsx! {
        div { style: "max-width: 400px;",
            Upload {
                drag: true,
                files: files(),
                tip: "支持 JPG、PNG 格式，单文件不超过 5MB".to_string(),
                onchange: move |f| files.set(f),
                div { style: "padding: 8px;", Button { variant: Variant::Primary, "选择文件" } }
            }
        }
    }
}

// ── Upload 文件大小校验演示 ──

#[component]
#[allow(non_snake_case)]
pub fn FileSizeValidationDemo() -> Element {
    let mut files = use_signal(|| Vec::new());

    rsx! {
        div { style: "max-width: 400px;",
            Upload {
                files: files(),
                tip: "文件大小不能超过 1KB".to_string(),
                onchange: move |f| files.set(f),
                Button { variant: Variant::Primary, "选择文件" }
            }
        }
    }
}

// ── Upload 限制文件类型演示 ──

#[component]
#[allow(non_snake_case)]
pub fn FileTypeLimitDemo() -> Element {
    let mut files = use_signal(|| Vec::new());

    rsx! {
        div { style: "max-width: 400px;",
            Upload {
                files: files(),
                accept: "image/*".to_string(),
                tip: "仅支持图片格式".to_string(),
                onchange: move |f| files.set(f),
                div { style: "padding: 8px;", Button { variant: Variant::Primary, size: Size::Sm, "图片上传" } }
            }
        }
    }
}

// ── Upload 自定义错误提示演示 ──

#[component]
#[allow(non_snake_case)]
pub fn CustomErrorFormatDemo() -> Element {
    let mut files = use_signal(|| Vec::new());

    rsx! {
        div { style: "max-width: 400px;",
            Upload {
                files: files(),
                tip: "超出 1KB 会显示自定义提示".to_string(),
                onchange: move |f| files.set(f),
                Button { variant: Variant::Primary, "选择文件" }
            }
        }
    }
}

// ── Upload 自定义错误处理演示 ──

#[component]
#[allow(non_snake_case)]
pub fn CustomErrorHandlerDemo() -> Element {
    let mut files = use_signal(|| Vec::new());

    rsx! {
        div { style: "max-width: 400px;",
            Upload {
                files: files(),
                tip: "选择文件上传".to_string(),
                onchange: move |f| files.set(f),
                Button { variant: Variant::Primary, "选择文件" }
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn BasicFormDemo() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    rsx! {
        Form {
            FormItem { label: "用户名".to_string(), required: true,
                Input {
                    placeholder: "请输入用户名",
                    value: username(),
                    oninput: move |v: String| username.set(v),
                }
            }
            FormItem { label: "密码".to_string(), required: true,
                Input {
                    r#type: "password",
                    placeholder: "请输入密码",
                    value: password(),
                    oninput: move |v: String| password.set(v),
                }
            }
            FormItem {
                Button { variant: Variant::Primary, "提交" }
            }
        }
    }
}

// ── Tree 演示 ──

#[component]
#[allow(non_snake_case)]
pub fn BasicTreeDemo() -> Element {
    let tree_data = vec![
        TreeNode {
            node_key: "1".to_string(),
            title: "一级节点 1".to_string(),
            child_nodes: vec![
                TreeNode { node_key: "1-1".to_string(), title: "二级节点 1-1".to_string(), ..Default::default() },
                TreeNode {
                    node_key: "1-2".to_string(),
                    title: "二级节点 1-2".to_string(),
                    child_nodes: vec![
                        TreeNode { node_key: "1-2-1".to_string(), title: "三级节点 1-2-1".to_string(), ..Default::default() },
                    ],
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        TreeNode {
            node_key: "2".to_string(),
            title: "一级节点 2".to_string(),
            child_nodes: vec![
                TreeNode { node_key: "2-1".to_string(), title: "二级节点 2-1".to_string(), ..Default::default() },
                TreeNode { node_key: "2-2".to_string(), title: "二级节点 2-2".to_string(), ..Default::default() },
            ],
            ..Default::default()
        },
    ];

    rsx! {
        Tree { data: tree_data, default_expand_all: true }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn SelectableTreeDemo() -> Element {
    let mut selected = use_signal(|| String::new());

    let tree_data = vec![
        TreeNode {
            node_key: "1".to_string(),
            title: "节点 1".to_string(),
            child_nodes: vec![
                TreeNode { node_key: "1-1".to_string(), title: "节点 1-1".to_string(), ..Default::default() },
                TreeNode { node_key: "1-2".to_string(), title: "节点 1-2".to_string(), ..Default::default() },
            ],
            ..Default::default()
        },
        TreeNode {
            node_key: "2".to_string(),
            title: "节点 2".to_string(),
            child_nodes: vec![
                TreeNode { node_key: "2-1".to_string(), title: "节点 2-1".to_string(), ..Default::default() },
            ],
            ..Default::default()
        },
    ];

    rsx! {
        Space { direction: Direction::Vertical, gap: "sm".to_string(),
            Tree {
                data: tree_data,
                selected_key: selected(),
                default_expand_all: true,
                onselect: move |k| selected.set(k),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "当前选中: "
                em { style: "font-style: normal; color: var(--ctrl-primary); font-weight: 500;",
                    if selected().is_empty() { "无" } else { "{selected()}" }
                }
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn CheckableTreeDemo() -> Element {
    let mut checked = use_signal(|| Vec::new());

    let tree_data = vec![
        TreeNode {
            node_key: "1".to_string(),
            title: "节点 1".to_string(),
            child_nodes: vec![
                TreeNode { node_key: "1-1".to_string(), title: "节点 1-1".to_string(), ..Default::default() },
                TreeNode { node_key: "1-2".to_string(), title: "节点 1-2".to_string(), ..Default::default() },
            ],
            ..Default::default()
        },
        TreeNode {
            node_key: "2".to_string(),
            title: "节点 2".to_string(),
            child_nodes: vec![
                TreeNode { node_key: "2-1".to_string(), title: "节点 2-1".to_string(), ..Default::default() },
            ],
            ..Default::default()
        },
    ];

    rsx! {
        Space { direction: Direction::Vertical, gap: "sm".to_string(),
            Tree {
                data: tree_data,
                checkable: true,
                checked_keys: checked(),
                default_expand_all: true,
                oncheck: move |keys| checked.set(keys),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "选中节点: "
                em { style: "font-style: normal; color: var(--ctrl-primary); font-weight: 500;",
                    if checked().is_empty() { "无" } else { "{checked().join(\", \")}" }
                }
            }
        }
    }
}

// ── DatePicker 演示 ──

#[component]
#[allow(non_snake_case)]
pub fn BasicDatePickerDemo() -> Element {
    let mut date = use_signal(|| String::new());

    rsx! {
        Space { direction: Direction::Vertical, gap: "sm".to_string(),
            DatePicker {
                value: date(),
                onchange: move |v| date.set(v),
            }
            span { style: "font-size: var(--ctrl-font-size-sm); color: var(--ctrl-text-secondary);",
                "选中日期: "
                em { style: "font-style: normal; color: var(--ctrl-primary); font-weight: 500;",
                    if date().is_empty() { "未选择" } else { "{date()}" }
                }
            }
        }
    }
}
