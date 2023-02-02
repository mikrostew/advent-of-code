use std::fmt;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::anychar;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::one_of;
use nom::combinator::eof;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::peek;
use nom::combinator::recognize;
use nom::multi::count;
use nom::multi::many1;
use nom::multi::many_m_n;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::IResult;

enum Markdown<'a> {
    H2(&'a str),
    Paragraph(Vec<Markdown<'a>>),
    Text(&'a str),
    InlineCode(Vec<Markdown<'a>>),
    Span(&'a str, &'a str),
    AnchorSpan(&'a str, &'a str, &'a str),
    Em(Vec<Markdown<'a>>),
    EmStar(&'a str),
    LinkRelative(&'a str, &'a str),
    LinkAbsolute(&'a str, &'a str),
    UnorderedList(Vec<Markdown<'a>>),
    ListItem(Vec<Markdown<'a>>),
    CodeBlock(Vec<Markdown<'a>>),
    CodeBlockEm(&'a str),
    CodeBlockSpan(&'a str, &'a str),
    CodeBlockText(&'a str),
    ParagraphSuccess(&'a str),
    Form(Vec<FormElement<'a>>),
    Discard,
}

enum FormElement<'a> {
    Text(&'a str),
    Input(&'a str),
}

// convert to markdown formatting
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
            Markdown::Text(s) => write!(f, "{}", normalize_whitespace(s)),
            Markdown::InlineCode(vm) => {
                write!(
                    f,
                    "{}",
                    match vm.len() {
                        1 => match &vm[0] {
                            Markdown::Text(s) => format!("`{}`", replace_html_char_codes(s)),
                            // expecting only one thing in the <em>
                            Markdown::Em(vm) =>
                                format!("**`{}`**", replace_html_char_codes(&vm[0].to_string())),
                            _ => unreachable!("inline code only contains text or <em>"),
                        },
                        _ => format!(
                            "<code>{}</code>",
                            vm.into_iter()
                                .map(|m| match m {
                                    Markdown::Text(s) => format!("{}", replace_html_char_codes(s)),
                                    // expecting only one thing in the <em>
                                    Markdown::Em(em_vm) => format!(
                                        "<em>{}</em>",
                                        replace_html_char_codes(&em_vm[0].to_string())
                                    ),
                                    _ => unreachable!("inline code only contains text or <em>"),
                                })
                                .collect::<String>()
                        ),
                    }
                )
            }
            // spans are used for easter eggs
            Markdown::Span(title, text) => write!(f, "[{}](# \"{}\")", text, title),
            Markdown::AnchorSpan(href, title, text) => {
                write!(f, "[{}]({} \"{}\")", text, href, title)
            }
            Markdown::Em(vm) => write!(
                f,
                "**{}**",
                replace_asterisks(&vm.into_iter().map(|m| m.to_string()).collect::<String>())
            ),
            Markdown::EmStar(s) => write!(f, "***{}***", s),
            Markdown::LinkRelative(href, text) => write!(f, "[{}]({})", text, convert_href(href)),
            Markdown::LinkAbsolute(href, text) => write!(f, "[{}]({})", text, href),
            Markdown::UnorderedList(vm) => write!(
                f,
                "{}\n\n",
                vm.into_iter()
                    .map(|li| li.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            Markdown::ListItem(vm) => write!(
                f,
                "* {}",
                vm.into_iter()
                    .map(|m| match m {
                        // only supports one level of nested list
                        Markdown::UnorderedList(vm) => format!(
                            "\n{}",
                            vm.into_iter()
                                .map(|li| format!("    {}", li))
                                .collect::<Vec<String>>()
                                .join("\n")
                        ),
                        _ => m.to_string(),
                    })
                    .collect::<String>()
            ),
            Markdown::CodeBlock(vm) => {
                let mut has_html = false;
                let contents = vm
                    .into_iter()
                    .map(|m| match m {
                        Markdown::CodeBlockEm(s) => {
                            has_html = true;
                            format!("<em>{}</em>", s.to_string())
                        }
                        Markdown::CodeBlockSpan(title, text) => {
                            has_html = true;
                            format!("<a href=\"#\" alt=\"{}\">{}</a>", title, text)
                        }
                        Markdown::CodeBlockText(s) => s.to_string(),
                        _ => unreachable!("only em and text in code blocks"),
                    })
                    .collect::<String>();

                if has_html {
                    write!(
                        f,
                        "<pre><code>\n{}</code></pre>\n\n",
                        replace_html_char_codes(&contents)
                    )
                } else {
                    write!(f, "```\n{}```\n\n", replace_html_char_codes(&contents))
                }
            }
            Markdown::CodeBlockEm(s) => write!(f, "<em>{}</em>", s),
            Markdown::CodeBlockSpan(title, text) => write!(f, "[{}](# \"{}\")", text, title),
            Markdown::CodeBlockText(s) => write!(f, "{}", s),
            Markdown::ParagraphSuccess(s) => write!(f, "**{} **\n\n", replace_asterisks(s)),
            Markdown::Form(vfe) => write!(
                f,
                "{}\n\n",
                vfe.into_iter().map(|m| m.to_string()).collect::<String>()
            ),
            Markdown::Discard => write!(f, ""),
        }
    }
}

impl fmt::Display for FormElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormElement::Text(s) => write!(f, "{}", normalize_whitespace(s)),
            FormElement::Input(s) => write!(f, "[{}](#)", s),
        }
    }
}

// table here: http://rabbit.eng.miami.edu/info/htmlchars.html
fn replace_html_char_codes(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
}

// so that asterisks inside of markdown bold ** don't mess things up
fn replace_asterisks(s: &str) -> String {
    s.replace("*", "&ast;")
}

// convert all whitespace (spaces/tabs/newlines/etc) into single spaces,
// preserving any whitespace at the beginning or ending of the string
fn normalize_whitespace(s: &str) -> String {
    let start_space = if s.starts_with(is_space) { " " } else { "" };
    let end_space = if s.ends_with(is_space) { " " } else { "" };
    format!(
        "{}{}{}",
        start_space,
        s.split_whitespace()
            .into_iter()
            .collect::<Vec<&str>>()
            .join(" "),
        end_space
    )
}
fn is_space(c: char) -> bool {
    match c {
        ' ' | '\t' | '\n' | '\r' => true,
        _ => false,
    }
}

fn convert_href(href: &str) -> String {
    if let Ok((_, (year, day))) = match_day_url(href) {
        format!("../../{}/descriptions/day{}.md", year, day)
    } else if let Ok((_, day)) = one_or_two_digits(href) {
        format!("./day{}.md", day)
    } else {
        println!("{}", href);
        panic!("fuck");
    }
}

fn match_day_url(input: &str) -> IResult<&str, (usize, usize)> {
    map(
        tuple((tag("/"), four_digits, tag("/day/"), one_or_two_digits, eof)),
        |(_, year, _, day, _)| {
            return (year, day);
        },
    )(input)
}

fn four_digits(input: &str) -> IResult<&str, usize> {
    map_res(recognize(count(one_of("0123456789"), 4)), |n| {
        usize::from_str_radix(n, 10)
    })(input)
}
fn one_or_two_digits(input: &str) -> IResult<&str, usize> {
    map_res(recognize(many_m_n(1, 2, one_of("0123456789"))), |n| {
        usize::from_str_radix(n, 10)
    })(input)
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
        ulist,
        code_block,
        paragraph_success,
        form,
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
fn whitespace_opt(input: &str) -> IResult<&str, &str> {
    multispace0(input)
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
    // sometimes the closing </p> is missing
    map(
        delimited(
            tag("<p>"),
            paragraph_contents,
            alt((tag("</p>"), peek(tag("<p>")))),
        ),
        |contents| Markdown::Paragraph(contents),
    )(input)
}
fn paragraph_contents(input: &str) -> IResult<&str, Vec<Markdown>> {
    many1(alt((
        p_a_span, p_code, p_span, p_em, p_em_star, p_a, p_text,
    )))(input)
}
// <a href=\"https://www.youtube.com/watch?v=sJFnWZH5FXc\" target=\"_blank\"><span title=\"While you're at it, go watch everything else by Retro Game Mechanics Explained, too.\">timing</span></a>
fn p_a_span(input: &str) -> IResult<&str, Markdown> {
    map(
        delimited(
            tag("<a href=\""),
            tuple((
                take_until1("\""),
                tag("\" target=\"_blank\"><span title=\""),
                take_until1("\""),
                tag("\">"),
                take_until1("</span>"),
            )),
            tag("</span></a>"),
        ),
        |(href, _, title, _, text)| Markdown::AnchorSpan(href, title, text),
    )(input)
}
fn p_code(input: &str) -> IResult<&str, Markdown> {
    // sometimes there are <em> sections in the inline code
    map(
        delimited(tag("<code>"), many1(alt((p_text, p_em))), tag("</code>")),
        |vm| Markdown::InlineCode(vm),
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
// TODO: I want to split this one up, for top level and inside <code>
fn p_em(input: &str) -> IResult<&str, Markdown> {
    // <em> can contain <code> and <span> in paragraphs
    map(
        delimited(
            tag("<em>"),
            many1(alt((p_text, p_code, p_span))),
            tag("</em>"),
        ),
        |vm| Markdown::Em(vm),
    )(input)
}
fn p_em_star(input: &str) -> IResult<&str, Markdown> {
    map(
        delimited(
            tag("<em class=\"star\">"),
            take_until1("</em>"),
            tag("</em>"),
        ),
        |s| Markdown::EmStar(s),
    )(input)
}
fn p_a(input: &str) -> IResult<&str, Markdown> {
    alt((p_a_relative, p_a_absolute))(input)
}
fn p_a_relative(input: &str) -> IResult<&str, Markdown> {
    map(
        delimited(
            tag("<a href=\""),
            separated_pair(take_until1("\""), tag("\">"), take_until1("</a>")),
            tag("</a>"),
        ),
        |(href, text)| Markdown::LinkRelative(href, text),
    )(input)
}
fn p_a_absolute(input: &str) -> IResult<&str, Markdown> {
    map(
        delimited(
            tag("<a href=\""),
            separated_pair(
                take_until1("\""),
                tag("\" target=\"_blank\">"),
                take_until1("</a>"),
            ),
            tag("</a>"),
        ),
        |(href, text)| Markdown::LinkAbsolute(href, text),
    )(input)
}
fn p_text(input: &str) -> IResult<&str, Markdown> {
    map(take_until1("<"), |text| Markdown::Text(text))(input)
}

// unordered list
fn ulist(input: &str) -> IResult<&str, Markdown> {
    map(
        delimited(tag("<ul>"), many1(list_item), tag("</ul>")),
        |vm| Markdown::UnorderedList(vm),
    )(input)
}
fn list_item(input: &str) -> IResult<&str, Markdown> {
    map(
        tuple((
            whitespace_opt,
            delimited(
                tag("<li>"),
                many1(alt((ulist, p_code, p_span, p_em, p_em_star, p_a, p_text))),
                tag("</li>"),
            ),
            whitespace_opt,
        )),
        |(_, vm, _)| Markdown::ListItem(vm),
    )(input)
}

// code blocks
fn code_block(input: &str) -> IResult<&str, Markdown> {
    // there can be <em> and <span> inside these
    map(
        delimited(
            tag("<pre><code>"),
            many1(alt((code_block_em, code_block_span, code_block_text))),
            // sometimes this is backwards
            alt((tag("</code></pre>"), tag("</pre></code>"))),
        ),
        |vm| Markdown::CodeBlock(vm),
    )(input)
}
// TODO: these should be a separate enum
fn code_block_em(input: &str) -> IResult<&str, Markdown> {
    map(
        delimited(tag("<em>"), take_until1("</em>"), tag("</em>")),
        |s| Markdown::CodeBlockEm(s),
    )(input)
}
fn code_block_span(input: &str) -> IResult<&str, Markdown> {
    map(
        delimited(
            tag("<span title=\""),
            separated_pair(take_until1("\""), tag("\">"), take_until1("</span>")),
            tag("</span>"),
        ),
        |(title, text)| Markdown::CodeBlockSpan(title, text),
    )(input)
}
fn code_block_text(input: &str) -> IResult<&str, Markdown> {
    map(take_until1("<"), |text| Markdown::CodeBlockText(text))(input)
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

// form to submit on day 25
//
// <form method=\"post\" action=\"25/answer\">
// <input type=\"hidden\" name=\"level\" value=\"2\"/><input type=\"hidden\" name=\"answer\" value=\"0\"/>
// <p>
// If you like, you can <input type=\"submit\" value=\"[Start The Blender Again]\"/>.
// </p>
// </form>
fn form(input: &str) -> IResult<&str, Markdown> {
    map(
        delimited(
            tag("<form method=\"post\" action=\"25/answer\">"),
            preceded(
                count(form_input_ignore, 2),
                delimited(tag("<p>"), many1(alt((form_text, form_input))), tag("</p>")),
            ),
            tag("</form>"),
        ),
        |vfe| Markdown::Form(vfe),
    )(input)
}

fn form_text(input: &str) -> IResult<&str, FormElement> {
    map(take_until1("<"), |t| FormElement::Text(t))(input)
}

fn form_input(input: &str) -> IResult<&str, FormElement> {
    map(
        delimited(
            tag("<input type=\"submit\" value=\""),
            take_until1("\""),
            tag("\"/>"),
        ),
        |t| FormElement::Input(t),
    )(input)
}

// <input type=\"hidden\" name=\"level\" value=\"2\"/>
fn form_input_ignore(input: &str) -> IResult<&str, &str> {
    recognize(delimited(
        tag("<input "),
        separated_list1(
            tag(" "),
            separated_pair(
                alpha1,
                tag("="),
                delimited(tag("\""), alphanumeric1, tag("\"")),
            ),
        ),
        tag("/>"),
    ))(input)
}
