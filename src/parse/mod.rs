use scraper::element_ref::Select;
use scraper::ElementRef;
use scraper::Selector;

use crate::prelude::*;

pub mod driver;
pub mod race;

pub(crate) fn next_inner_html<'a>(i: &mut impl Iterator<Item = ElementRef<'a>>) -> Result<String> {
    let s = i
        .next()
        .ok_or(anyhow::anyhow!("expected inner html"))?
        .inner_html()
        .trim()
        .to_string();
    Ok(s)
}

pub(crate) struct HtmlTable<'a> {
    inner: ElementRef<'a>,

    s_content: Selector,
}

impl<'a> HtmlTable<'a> {
    fn new(elem: ElementRef<'a>) -> Self {
        let s_content = Selector::parse("tbody>tr").unwrap();
        Self {
            inner: elem,
            s_content,
        }
    }

    pub(crate) fn parse<S: AsRef<str> + std::fmt::Display>(
        elem: &'a ElementRef,
        selectors: S,
    ) -> Result<Self> {
        let selector = Selector::parse(selectors.as_ref()).unwrap();
        let inner = elem
            .select(&selector)
            .next()
            .with_context(|| format!("selecting table at: `{selectors}`"))?;
        Ok(Self::new(inner))
    }

    pub(crate) fn rows(&self) -> Select {
        return self.inner.select(&self.s_content);
    }
}
