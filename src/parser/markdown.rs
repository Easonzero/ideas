use crate::item::Item;

#[allow(dead_code)]
pub fn parse(item: &Item) -> String {
    let summary = format!(
        r#"
----
[**{}**]: {}
----
## Summary
> {}
----
"#,
        item.ty.desc, item.ty.status, item.summary
    );
    let detail = match &item.detail {
        Some(ref detail) => format!(
            r#"
## Detail
> {}
----
"#,
            detail
        ),
        None => "".to_owned(),
    };
    let url = match &item.url {
        Some(ref url) => format!(
            r#"
## Reference
* {url}
----
"#,
            url = url
        ),
        None => "".to_owned(),
    };
    summary + &detail + &url
}
