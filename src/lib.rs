use rss::Channel;
use std::str::FromStr;

pub async fn get_updated_feed() -> Result<Channel, Box<dyn std::error::Error>> {
    let feed = get_feed("https://stpaulsharringay.com/wp-content/uploads/podcast.xml").await?;
    let mut channel = Channel::from_str(&feed)?;
    for item in channel.items_mut() {
        let sermon_title = item.title.as_ref();
        let passage = item.description.as_ref().map(|s| {
            s.trim()
                .replace(": ", ":")
                .replace(" :", ":")
                .replace("- ", "-")
                .replace(" -", "-")
        });

        let new_title: Option<String> = match (sermon_title, passage) {
            (Some(title), Some(passage)) => Some(format!("{title} ({passage})")),
            (None, Some(passage)) => Some(format!("{}", &passage)),
            (Some(title), None) => Some(format!("{}", &title)),
            (None, None) => None,
        };
        item.title = new_title;
        let author = item
            .itunes_ext
            .as_ref()
            .and_then(|e| e.author.as_ref())
            .map(|a| a.trim());
        item.description = author.map(|a| a.to_owned());
    }

    Ok(channel)
}

async fn get_feed(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = reqwest::get(url).await?.bytes().await?;
    let content = std::str::from_utf8(&bytes)?;
    Ok(replace_html_entities(content))
}

/// XML only supports 5 named entities. WordPress sometimes outputs HTML named
/// entities (e.g. &Icirc;) which are not valid XML. This converts known HTML
/// named entities to their Unicode equivalents so the XML parser can handle them.
pub(crate) fn replace_html_entities(xml: &str) -> String {
    const XML_ENTITIES: &[&str] = &["amp", "lt", "gt", "apos", "quot"];

    let mut result = String::with_capacity(xml.len());
    let mut rest = xml;

    while let Some(amp_pos) = rest.find('&') {
        result.push_str(&rest[..amp_pos]);
        rest = &rest[amp_pos + 1..];

        if let Some(semi_pos) = rest.find(';') {
            let name = &rest[..semi_pos];
            if XML_ENTITIES.contains(&name) || name.starts_with('#') {
                // Valid XML entity or numeric entity — leave alone
                result.push('&');
                result.push_str(name);
                result.push(';');
            } else if let Some(ch) = html_named_entity(name) {
                result.push(ch);
            } else {
                // Unknown entity — leave alone
                result.push('&');
                result.push_str(name);
                result.push(';');
            }
            rest = &rest[semi_pos + 1..];
        } else {
            // No closing semicolon — not an entity
            result.push('&');
        }
    }
    result.push_str(rest);
    result
}

pub(crate) fn html_named_entity(name: &str) -> Option<char> {
    match name {
        "Agrave" => Some('À'), "Aacute" => Some('Á'), "Acirc"  => Some('Â'),
        "Atilde" => Some('Ã'), "Auml"   => Some('Ä'), "Aring"  => Some('Å'),
        "AElig"  => Some('Æ'), "Ccedil" => Some('Ç'), "Egrave" => Some('È'),
        "Eacute" => Some('É'), "Ecirc"  => Some('Ê'), "Euml"   => Some('Ë'),
        "Igrave" => Some('Ì'), "Iacute" => Some('Í'), "Icirc"  => Some('Î'),
        "Iuml"   => Some('Ï'), "ETH"    => Some('Ð'), "Ntilde" => Some('Ñ'),
        "Ograve" => Some('Ò'), "Oacute" => Some('Ó'), "Ocirc"  => Some('Ô'),
        "Otilde" => Some('Õ'), "Ouml"   => Some('Ö'), "Oslash" => Some('Ø'),
        "Ugrave" => Some('Ù'), "Uacute" => Some('Ú'), "Ucirc"  => Some('Û'),
        "Uuml"   => Some('Ü'), "Yacute" => Some('Ý'), "THORN"  => Some('Þ'),
        "szlig"  => Some('ß'), "agrave" => Some('à'), "aacute" => Some('á'),
        "acirc"  => Some('â'), "atilde" => Some('ã'), "auml"   => Some('ä'),
        "aring"  => Some('å'), "aelig"  => Some('æ'), "ccedil" => Some('ç'),
        "egrave" => Some('è'), "eacute" => Some('é'), "ecirc"  => Some('ê'),
        "euml"   => Some('ë'), "igrave" => Some('ì'), "iacute" => Some('í'),
        "icirc"  => Some('î'), "iuml"   => Some('ï'), "eth"    => Some('ð'),
        "ntilde" => Some('ñ'), "ograve" => Some('ò'), "oacute" => Some('ó'),
        "ocirc"  => Some('ô'), "otilde" => Some('õ'), "ouml"   => Some('ö'),
        "oslash" => Some('ø'), "ugrave" => Some('ù'), "uacute" => Some('ú'),
        "ucirc"  => Some('û'), "uuml"   => Some('ü'), "yacute" => Some('ý'),
        "thorn"  => Some('þ'), "yuml"   => Some('ÿ'), "nbsp"   => Some('\u{00A0}'),
        "iexcl"  => Some('¡'), "cent"   => Some('¢'), "pound"  => Some('£'),
        "curren" => Some('¤'), "yen"    => Some('¥'), "brvbar" => Some('¦'),
        "sect"   => Some('§'), "uml"    => Some('¨'), "copy"   => Some('©'),
        "ordf"   => Some('ª'), "laquo"  => Some('«'), "not"    => Some('¬'),
        "reg"    => Some('®'), "macr"   => Some('¯'), "deg"    => Some('°'),
        "plusmn" => Some('±'), "sup2"   => Some('²'), "sup3"   => Some('³'),
        "acute"  => Some('´'), "micro"  => Some('µ'), "para"   => Some('¶'),
        "middot" => Some('·'), "cedil"  => Some('¸'), "sup1"   => Some('¹'),
        "ordm"   => Some('º'), "raquo"  => Some('»'), "frac14" => Some('¼'),
        "frac12" => Some('½'), "frac34" => Some('¾'), "iquest" => Some('¿'),
        "times"  => Some('×'), "divide" => Some('÷'),
        "ndash"  => Some('–'), "mdash"  => Some('—'), "lsquo"  => Some('\u{2018}'),
        "rsquo"  => Some('\u{2019}'), "ldquo" => Some('\u{201C}'),
        "rdquo"  => Some('\u{201D}'), "hellip" => Some('…'),
        "euro"   => Some('€'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rss::Channel;
    use std::str::FromStr;

    // Regression test: WordPress encoded "Îbrâhîm" using HTML named entities
    // (e.g. &Icirc;) which are not valid XML, causing the RSS parser to 500.
    #[test]
    fn test_replace_html_entities_decodes_icirc() {
        let input = "Jeremy &Icirc;br&acirc;h&icirc;m Fowler";
        assert_eq!(replace_html_entities(input), "Jeremy Îbrâhîm Fowler");
    }

    #[test]
    fn test_replace_html_entities_preserves_xml_entities() {
        let input = "&amp; &lt; &gt; &apos; &quot;";
        assert_eq!(replace_html_entities(input), "&amp; &lt; &gt; &apos; &quot;");
    }

    #[test]
    fn test_replace_html_entities_preserves_numeric_entities() {
        let input = "&#206; &#xCE;";
        assert_eq!(replace_html_entities(input), "&#206; &#xCE;");
    }

    #[test]
    fn test_feed_with_html_entity_parses_successfully() {
        // Minimal RSS feed replicating the WordPress bug: &Icirc; in itunes:author
        let feed = r#"<?xml version="1.0" encoding="utf-8"?>
<rss version="2.0" xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd">
  <channel>
    <title>Test Podcast</title>
    <link>https://example.com</link>
    <description>Test</description>
    <item>
      <title>Test Sermon</title>
      <description>Acts 2:1-11</description>
      <itunes:author>Jeremy &Icirc;br&acirc;h&icirc;m Fowler</itunes:author>
    </item>
  </channel>
</rss>"#;

        // Without preprocessing this would fail with an XML escape error
        let preprocessed = replace_html_entities(feed);
        let channel = Channel::from_str(&preprocessed).expect("should parse after entity replacement");
        let author = channel.items()[0]
            .itunes_ext()
            .and_then(|e| e.author())
            .unwrap();
        assert_eq!(author, "Jeremy Îbrâhîm Fowler");
    }
}
