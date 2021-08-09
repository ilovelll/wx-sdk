use roxmltree::Node;

use crate::{error::SdkError, SdkResult};

pub fn get_node_by_tag<'a, 'b>(node: &'a Node, tag_name: &'b str) -> SdkResult<Node<'a, 'a>> {
    node.descendants()
        .find(|n| n.has_tag_name(tag_name))
        .ok_or_else(|| {
            SdkError::InvalidParams(format!(
                "Parse XML msg from wechat error: tag `{}` invalid",
                tag_name
            ))
        })
}

pub fn get_text_from_root<'a>(node: &Node<'a, 'a>, tag_name: &str) -> SdkResult<&'a str> {
    node.descendants()
        .find(|n| n.has_tag_name(tag_name))
        .map(|n| n.text())
        .flatten()
        .ok_or_else(|| {
            SdkError::InvalidParams(format!(
                "Parse XML msg from wechat error: tag `{}` text content is none",
                node.tag_name().name()
            ))
        })
}

pub fn get_number_from_root<'a, T: std::str::FromStr>(
    node: &Node<'a, 'a>,
    tag_name: &str,
) -> SdkResult<T> {
    node.descendants()
        .find(|n| n.has_tag_name(tag_name))
        .map(|n| n.text())
        .flatten()
        .map(|s| s.parse::<T>().ok())
        .flatten()
        .ok_or_else(|| {
            SdkError::InvalidParams(format!(
                "Parse XML msg from wechat error: tag `{}` should be number",
                tag_name
            ))
        })
}
