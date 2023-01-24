use std::fmt;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
// use nom::character::complete::alphanumeric1;
use nom::character::complete::anychar;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::combinator::recognize;
use nom::multi::many1;
// use nom::multi::many_till;
use nom::sequence::delimited;
// use nom::sequence::preceded;
use nom::sequence::separated_pair;
// use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

enum Markdown<'a> {
    H2(&'a str),
    Paragraph(Vec<Markdown<'a>>),
    Text(&'a str),
    InlineCode(&'a str),
    // title text, spanned text
    Span(&'a str, &'a str),
    Em(&'a str),
    CodeBlock(&'a str),
    ParagraphSuccess(&'a str),
    Discard,
}

impl fmt::Display for Markdown<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Markdown::H2(s) => write!(f, "# {}\n\n", s),
            Markdown::Paragraph(vec_of_md) => write!(
                f,
                "{}\n\n",
                vec_of_md
                    .into_iter()
                    .map(|m| m.to_string())
                    .collect::<String>()
            ),
            Markdown::Text(s) => write!(f, "{}", s),
            Markdown::InlineCode(c) => write!(f, "`{}`", replace_html_char_codes(c)),
            // spans are used for easter eggs
            Markdown::Span(title, text) => write!(f, "[{}](# \"{}\")", text, title),
            Markdown::Em(s) => write!(f, "**{}**", s),
            Markdown::CodeBlock(s) => write!(f, "```\n{}```\n\n", replace_html_char_codes(s)),
            Markdown::ParagraphSuccess(s) => write!(f, "**{} **\n\n", s),
            Markdown::Discard => write!(f, ""),
        }
    }
}

// table here: http://rabbit.eng.miami.edu/info/htmlchars.html
fn replace_html_char_codes(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
}

pub(crate) fn html_to_md(html: &str) -> Result<String, String> {
    let (leftover, markdown) = match parse_html_to_md(html) {
        Ok((l, m)) => (l, m),
        Err(err) => {
            return Err(format!("Could not parse html: {}", err.to_string()));
        }
    };
    if leftover != "" {
        return Err(format!("Parse incomplete, leftover='{}'", leftover));
    }
    Ok(markdown)
}

fn parse_html_to_md(input: &str) -> IResult<&str, String> {
    map(
        tuple((before_main, main, after_main)),
        |(_bm, main_md, _am)| main_md,
    )(input)
}

fn before_main(input: &str) -> IResult<&str, &str> {
    recognize(tuple((take_until1("<main>"), tag("<main>"))))(input)
}

fn main(input: &str) -> IResult<&str, String> {
    map(main_md, |md: Vec<Markdown>| {
        md.into_iter().map(|m| m.to_string()).collect::<String>()
    })(input)
}

fn after_main(input: &str) -> IResult<&str, &str> {
    recognize(tuple((tag("</main>"), many1(anychar))))(input)
}

fn main_md(input: &str) -> IResult<&str, Vec<Markdown>> {
    many1(alt((
        discard,
        header,
        paragraph,
        code_block,
        paragraph_success,
    )))(input)
}

// things which will be discarded
fn discard(input: &str) -> IResult<&str, Markdown> {
    map(
        alt((
            whitespace,
            style,
            article_start,
            article_end,
            admire,
            get_input,
            sharing,
        )),
        |_| Markdown::Discard,
    )(input)
}
fn style(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        tag("<style>"),
        take_until1("</style>"),
        tag("</style>"),
    )))(input)
}
fn article_start(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        tag("<article class=\""),
        take_until1("\""),
        tag("\">"),
    )))(input)
}
fn article_end(input: &str) -> IResult<&str, &str> {
    tag("</article>")(input)
}
fn whitespace(input: &str) -> IResult<&str, &str> {
    multispace1(input)
}
fn admire(input: &str) -> IResult<&str, &str> {
    recognize(delimited(
        tag("<p>At this point, all that is left is for you to"),
        take_until1("</p>"),
        tag("</p>"),
    ))(input)
}
fn get_input(input: &str) -> IResult<&str, &str> {
    recognize(delimited(
        tag("<p>If you still want to see it, you can"),
        take_until1("</p>"),
        tag("</p>"),
    ))(input)
}
fn sharing(input: &str) -> IResult<&str, &str> {
    recognize(delimited(
        tag("<p>You can also <span class=\"share\">[Share<span"),
        take_until1("</span>]</span>"),
        tag("</span>]</span> this puzzle.</p>"),
    ))(input)
}

// headers
// (currently only h2)
fn header(input: &str) -> IResult<&str, Markdown> {
    map(
        delimited(
            alt((tag("<h2>"), tag("<h2 id=\"part2\">"))),
            take_until1("</h2>"),
            tag("</h2>"),
        ),
        |text| Markdown::H2(text),
    )(input)
}

// paragraphs
fn paragraph(input: &str) -> IResult<&str, Markdown> {
    map(
        delimited(tag("<p>"), paragraph_contents, tag("</p>")),
        |contents| Markdown::Paragraph(contents),
    )(input)
}
fn paragraph_contents(input: &str) -> IResult<&str, Vec<Markdown>> {
    many1(alt((p_code, p_span, p_em, p_text)))(input)
    //many1(alt((p_code, p_text)))(input)
}
fn p_code(input: &str) -> IResult<&str, Markdown> {
    map(
        delimited(tag("<code>"), take_until1("</code>"), tag("</code>")),
        |text| Markdown::InlineCode(text),
    )(input)
}
fn p_span(input: &str) -> IResult<&str, Markdown> {
    map(
        delimited(
            tag("<span title=\""),
            separated_pair(take_until1("\""), tag("\">"), take_until1("</span>")),
            tag("</span>"),
        ),
        |(title, text)| Markdown::Span(title, text),
    )(input)
}
fn p_em(input: &str) -> IResult<&str, Markdown> {
    map(
        delimited(tag("<em>"), take_until1("</em>"), tag("</em>")),
        |text| Markdown::Em(text),
    )(input)
}
fn p_text(input: &str) -> IResult<&str, Markdown> {
    map(take_until1("<"), |text| Markdown::Text(text))(input)
}

// code blocks
fn code_block(input: &str) -> IResult<&str, Markdown> {
    map(
        delimited(
            tag("<pre><code>"),
            take_until1("</code></pre>"),
            tag("</code></pre>"),
        ),
        |text| Markdown::CodeBlock(text),
    )(input)
}

// paragraph with this success message
fn paragraph_success(input: &str) -> IResult<&str, Markdown> {
    map(
        delimited(
            tag("<p class=\"day-success\">"),
            take_until1("</p>"),
            tag("</p>"),
        ),
        |text| Markdown::ParagraphSuccess(text),
    )(input)
}
