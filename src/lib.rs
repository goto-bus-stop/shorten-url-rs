use std::borrow::Cow;

/// Shorten a URL to `max_len` bytes.
pub fn shorten(input: &str, max_len: usize) -> Cow<'_, str> {
    if input.len() < max_len {
        return input.into();
    }

    let rest = input;
    let (scheme, rest) = if let Some(index) = rest.find("://") {
        input.split_at(index + "://".len())
    } else {
        ("", input)
    };

    let (host, rest) = if let Some(index) = rest.find('/') {
        rest.split_at(index)
    } else {
        (rest, "")
    };

    let (path, query) = if let Some(index) = rest.find('?') {
        rest.split_at(index)
    } else {
        (rest, "")
    };

    let mut new_len = scheme.len() + host.len() + path.len();
    let mut path_parts = if path.is_empty() {
        vec![]
    } else {
        path[1..].split('/').collect::<Vec<_>>()
    };
    let mut path_subst_index = None;
    while !path_parts.is_empty() && new_len > max_len - 2 {
        let splice_index = (path_parts.len() / 2).saturating_sub(1);
        let removed_part = path_parts.remove(splice_index);
        new_len -= removed_part.len() + 1 /* the / */;
        path_subst_index = Some(splice_index);
    }

    let available_len = (max_len - 2).saturating_sub(new_len);
    let truncated_query = if query.len() > available_len {
        let trunc_len = if let Some(amp) = query[0..available_len].rfind('&') {
            amp + 1
        } else {
            1
        };
        Some(&query[0..trunc_len])
    } else {
        None
    };

    // If we didn't modify anything, return the original
    if path_subst_index.is_none() && truncated_query.is_none() {
        if input.len() > max_len {
            let mut new_url = String::with_capacity(max_len);
            new_url.push_str(&input[0..max_len - 1]);
            new_url.push('…');
            return new_url.into();
        }

        return input.into();
    }

    if let Some(index) = path_subst_index {
        path_parts.insert(index, "…");
    }

    let mut new_url = String::with_capacity(max_len);

    new_url.push_str(scheme);
    new_url.push_str(host);
    for part in path_parts {
        new_url.push('/');
        new_url.push_str(part);
    }
    match truncated_query {
        Some(truncated) => {
            new_url.push_str(truncated);
            new_url.push('…');
        }
        None => new_url.push_str(query),
    }

    new_url.into()
}

#[cfg(test)]
mod tests {
    use super::shorten;
    use std::borrow::Cow;

    #[test]
    fn shortens_paths() {
        assert_eq!(
            shorten(
                "https://www.vpro.nl/programmas/gliphoeve/documentaire-intro.html",
                50
            ),
            "https://www.vpro.nl/…/documentaire-intro.html"
        );
        assert_eq!(
            shorten(
                "https://discordapp.com/channels/317475976369930241/317475976369930241",
                25
            ),
            "https://discordapp.com/…"
        );
        assert_eq!(
            shorten(
                "http://example.com/ultra/cool/page/that-is-really-deeply/nested/",
                30
            ),
            "http://example.com/…/nested/"
        );
    }

    #[test]
    fn shortens_queries() {
        assert_eq!(
            shorten("http://www.blahblah.com/unpragmatic-thoughts/?p=1738", 50),
            "http://www.blahblah.com/unpragmatic-thoughts/?…"
        );
        assert_eq!(
            shorten("https://www.reddit.com/?count=25&after=t3_76zjp1", 40),
            "https://www.reddit.com/?count=25&…"
        );
    }

    #[test]
    fn shortens_hosts() {
        assert!(matches!(
            shorten("https://www.thisisasuperlonghostname.co.uk", 35),
            Cow::Owned(_)
        ));
        assert_eq!(
            shorten("https://www.thisisasuperlonghostname.co.uk", 35),
            "https://www.thisisasuperlonghostna…"
        );
    }
}
