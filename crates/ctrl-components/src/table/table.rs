use dioxus::prelude::*;

/// Table 列定义
#[derive(Clone, PartialEq)]
pub struct TableColumn {
    /// 列标题
    pub title: String,
    /// 列宽
    #[allow(dead_code)]
    pub width: Option<String>,
    /// 对齐方式
    #[allow(dead_code)]
    pub align: Option<String>,
}

/// Table 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct TableProps {
    /// 列定义
    #[props(default = Vec::new())]
    pub columns: Vec<TableColumn>,

    /// 行数据（每行是一个 Vec<String>）
    #[props(default = Vec::new())]
    pub data: Vec<Vec<String>>,

    /// 是否显示斑马纹
    #[props(default = false)]
    pub striped: bool,

    /// 是否显示边框
    #[props(default = true)]
    pub bordered: bool,

    /// 自定义类名
    #[props(default = "".to_string())]
    pub class: String,

    /// 自定义样式
    #[props(default = "".to_string())]
    pub style: String,
}

/// Table 表格组件
#[allow(non_snake_case)]
pub fn Table(props: TableProps) -> Element {
    const CSS: &str = include_str!("../../assets/table.css");
    let mut table_classes = vec!["ctrl-table".to_string()];
    if props.bordered {
        table_classes.push("ctrl-table--bordered".into());
    }
    let table_class = table_classes.join(" ");

    let mut wrap_class = "ctrl-table__wrap".to_string();
    if !props.class.is_empty() {
        wrap_class = format!("{} {}", wrap_class, props.class);
    }

    // 提前克隆所有需要的数据
    let data = props.data.clone();
    let columns_count = props.columns.len();
    let cols = props.columns.clone();
    let striped = props.striped;

    rsx! {
        style { {CSS} }
        div {
            class: "{wrap_class}",
            style: if !props.style.is_empty() { props.style.as_str() } else { "" },
            table {
                class: "{table_class}",
                thead {
                    tr {
                        {
                            cols.iter().map(|col| {
                                rsx! {
                                    th {
                                        class: "ctrl-table__th",
                                        "{col.title}"
                                    }
                                }
                            })
                        }
                    }
                }
                tbody {
                    {
                        data.iter().enumerate().map(move |(row_idx, row)| {
                            let mut tr_classes = vec!["ctrl-table__tr".to_string()];
                            if striped && row_idx % 2 == 1 {
                                tr_classes.push("ctrl-table__tr--striped".into());
                            }
                            let tr_class = tr_classes.join(" ");

                            let row_data = row.iter().take(columns_count).cloned().collect::<Vec<_>>();
                            rsx! {
                                tr {
                                    key: "{row_idx}",
                                    class: "{tr_class}",
                                    {
                                        row_data.into_iter().map(|cell| {
                                            rsx! {
                                                td {
                                                    class: "ctrl-table__td",
                                                    "{cell}"
                                                }
                                            }
                                        })
                                    }
                                }
                            }
                        })
                    }
                }
            }
        }
    }
}

impl Default for TableColumn {
    fn default() -> Self {
        Self {
            title: String::new(),
            width: None,
            align: None,
        }
    }
}
