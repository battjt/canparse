//! Nom-based parsers for Entry types

use super::*;
use nom::{digit, float, line_ending, space, space0, AsChar};
use std::str::FromStr;

// TODO: convert `tag!(" ")` to `space`

named! {
    quoted_str<&str, String>,
    map!(
        delimited!(
            tag!("\""),
            escaped_transform!(
                none_of!("\\\""),
                '\\',
                alt!(
                    tag!("\\") => { |_| "\\" }
                  | tag!("\"") => { |_| "\"" }
                )),
            tag!("\"")),
        |data| data)
}

named!(pub entry<&str, Entry>, alt!(
    version                => { Entry::Version } |
    bus_configuration      => { Entry::BusConfiguration } |
    message_definition     => { Entry::MessageDefinition } |
    message_description    => { Entry::MessageDescription } |
    message_attribute      => { Entry::MessageAttribute } |
    signal_definition      => { Entry::SignalDefinition } |
    signal_description     => { Entry::SignalDescription } |
    signal_attribute       => { Entry::SignalAttribute } |
    unknown                => { Entry::Unknown }
));

named!(pub unknown<&str, String>,
    do_parse!(
        // FIXME: many0!(quoted_str) >> line_ending
        data: take_until_either!("\r\n") >>
        line_ending >>
        ( data.to_string() )
    )
);

named!(pub version<&str, Version>,
    do_parse!(
        tag!("VERSION")   >>
        tag!(" ")   >>
        data: quoted_str >>
        line_ending >>
        ( Version(data) )
    )
);

named!(pub bus_configuration<&str, BusConfiguration>,
    do_parse!(
        tag!("BS_:")   >>
        tag!(" ")   >>
        data: map_res!(
            take_until_either!("\r\n"),
            FromStr::from_str) >>
        line_ending >>
        ( BusConfiguration(data) )
    )
);

fn is_alphanumeric_extended(c: char) -> bool {
    c.is_alphanum() || c == '_'
}

// FIXME: `space` isn't really correct since there should only be ONE (probably need alt)
named!(pub message_definition<&str, MessageDefinition>,
    do_parse!(
        tag!("BO_")   >>
        space >>
        id: map_res!(
            digit,
            FromStr::from_str) >>
        space >>
        name: take_while!(is_alphanumeric_extended) >>
        space0 >>
        tag!(":")   >>
        space >>
        len: map_res!(
            digit,
            FromStr::from_str) >>
        space >>
        sending_node: take_until_either!(" \t\r\n") >>
        space0 >>
        line_ending >>
        ( MessageDefinition {
            id,
            name: name.into(),
            message_len: len,
            sending_node: sending_node.into(),
        } )
    )
);

named!(pub message_description<&str, MessageDescription>,
    do_parse!(
        tag!("CM_")   >>
        space >>
        tag!("BO_")   >>
        space >>
        id: map_res!(
            digit,
            FromStr::from_str) >>
        space >>
        description: quoted_str >>
        tag!(";") >>
        line_ending >>
        ( MessageDescription {
            id: id,
            signal_name: "".to_string(),
            description: description,
        } )
    )
);

named!(pub message_attribute<&str, MessageAttribute>,
    do_parse!(
        tag!("BA_")   >>
        space >>
        name: quoted_str >>
        space >>
        tag!("BO_")   >>
        space >>
        id: map_res!(
            digit,
            FromStr::from_str) >>
        space >>
        value: digit >>
        tag!(";") >>
        line_ending >>
        ( MessageAttribute {
            name: name,
            signal_name: "".to_string(),
            id: id,
            value: value.to_string()
        } )
    )
);

named!(pub signal_definition<&str, SignalDefinition>,
    do_parse!(
        space >>
        tag!("SG_") >>
        space >>
        name: take_until_either!(" \t") >>
        space >>
        tag!(":") >>
        space >>
        start_bit: map_res!(
            digit,
            FromStr::from_str) >>
        tag!("|") >>
        bit_len: map_res!(
            digit,
            FromStr::from_str) >>
        tag!("@") >>
        little_endian: map!(digit, |d| d == "1") >>
        signed: alt!(
            tag!("+") => { |_| false } |
            tag!("-") => { |_| true } ) >>
        space >>
        tag!("(") >>
        scale: float >>
        tag!(",") >>
        offset: float >>
        tag!(")") >>
        space >>
        tag!("[") >>
        min_value: float >>
        tag!("|") >>
        max_value: float >>
        tag!("]") >>
        space >>
        units: quoted_str >>
        space >>
        receiving_node: take_until_either!(" \t\r\n") >>
        line_ending >>
        ( SignalDefinition {
            name: name.to_string(),
            start_bit: start_bit,
            bit_len: bit_len,
            little_endian: little_endian,
            signed: signed,
            scale: scale,
            offset: offset,
            min_value: min_value,
            max_value: max_value,
            units: units,
            receiving_node: receiving_node.to_string(),
        } )
    )
);

named!(pub signal_description<&str, SignalDescription>,
    do_parse!(
        tag!("CM_")   >>
        space >>
        tag!("SG_")   >>
        space >>
        id: map_res!(
            digit,
            FromStr::from_str) >>
        space >>
        signal_name: take_until_either!(" \t") >>
        space >>
        description: quoted_str >>
        tag!(";") >>
        line_ending >>
        ( SignalDescription {
            id: id,
            signal_name: signal_name.to_string(),
            description: description
        } )
    )
);

named!(pub signal_attribute<&str, SignalAttribute>,
    do_parse!(
        tag!("BA_")   >>
        space >>
        name: quoted_str >>
        space >>
        tag!("SG_")   >>
        space >>
        id: map_res!(
            digit,
            FromStr::from_str) >>
        space >>
        signal_name: take_until_either!(" \t") >>
        space >>
        value: digit >>
        tag!(";") >>
        line_ending >>
        ( SignalAttribute {
            name: name,
            id: id,
            signal_name: signal_name.to_string(),
            value: value.to_string()
        } )
    )
);
