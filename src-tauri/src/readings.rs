use std::fs;

use minidom::{Element, NSChoice};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{path::{BaseDirectory, PathResolver}, Runtime, State};

use crate::bible::VerseRange;

pub const ROBERT_ROBERTS_PATH: &str = "resources/reading_plans/rr.xml";
pub const CHRONOLOGICAL_PATH: &str = "resources/reading_plans/chron.json";

pub struct ReadingsDatabase
{
    robert_roberts: Element,
    chronological: Value,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat,
    InvalidNumber,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reading 
{
    pub prefix: Option<u32>,
    pub book: String,
    pub chapter: u32,
    pub range: Option<VerseRange>
}

impl ReadingsDatabase
{
    pub fn new<R>(resolver: &PathResolver<R>) ->  Self 
        where R : Runtime
    {
        let rr_p = resolver.resolve(ROBERT_ROBERTS_PATH, BaseDirectory::Resource).unwrap();
        let rr_data = fs::read_to_string(rr_p).unwrap();
        let robert_roberts: Element = rr_data.parse().unwrap();

        let chron_p = resolver.resolve(CHRONOLOGICAL_PATH, BaseDirectory::Resource).unwrap();
        let chron_data = fs::read_to_string(chron_p).unwrap();
        let chronological: Value = serde_json::from_str(&chron_data).unwrap();

        Self 
        {
            robert_roberts,
            chronological,
        }
    }

    pub fn get_readings(&self, month: u32, day: u32, selected_reading: SelectedReading) -> Vec<Reading>
    {
        match selected_reading
        {
            SelectedReading::RobertRoberts => self.get_rr_readings(month, day),
            SelectedReading::Chronological => self.get_chronological_readings(month, day),
            SelectedReading::Proverbs => Self::get_pro_readings(day),
        }
    }

    fn get_chronological_readings(&self, month: u32, day: u32) -> Vec<Reading>
    {
        if month == 1 && day == 28 { return vec![]; } // fix for leap year

        const MONTH_LENGTHS: [u32; 12] = [
            31, // Jan
            28, // Feb
            31, // Mar
            30, // Apr
            31, // May
            30, // Jun
            31, // Jul
            31, // Aug
            30, // Sep
            31, // Oct
            30, // Nov
            31, // Dec
        ];

        let index = if month == 0 { day } else { MONTH_LENGTHS[0..(month as usize)].iter().sum::<u32>() + day };

        let day_readings = &self.chronological.get("data2").unwrap()
            .as_array().unwrap()[index as usize];

        let readings = day_readings.as_array().unwrap().iter()
            .map(|r| Self::parse(r.as_str().unwrap()).unwrap())
            .flatten()
            .collect::<Vec<_>>();

        readings
    }

    fn get_pro_readings(day: u32) -> Vec<Reading>
    {
        vec![Reading {
            prefix: None,
            book: "Proverbs".into(),
            chapter: day,
            range: None
        }]
    }

    fn get_rr_readings(&self, month: u32, day: u32) -> Vec<Reading>
    {
        if month == 1 && day == 28 { return vec![]; } // fix for leap year
        let month_readings = self.robert_roberts.children().find(|c| {
            c.is("month", NSChoice::Any) && 
            c.attr("num").is_some_and(|num| {
                num.parse::<u32>().unwrap() == month + 1 // dataset is not 0 based index
            })
        }).unwrap();

        let day_readings = month_readings.children().find(|c| {
            c.is("day", NSChoice::Any) &&
            c.attr("num").is_some_and(|num| {
                num.parse::<u32>().unwrap() == day + 1 // dataset is not 0 based index
            })
        }).unwrap();
        
        let readings: Vec<_> = day_readings.children().filter_map(|r| {
            if r.is("reading", NSChoice::Any)
            {
                Some(r.attr("passage").unwrap())
            }
            else 
            {
                None    
            }
        }).map(|r| {
            Self::parse(r).unwrap()
        }).flatten().collect();

        readings
    }

    // This was AI-ed, will change in future
    fn parse(text: &str) -> Result<Vec<Reading>, ParseError> 
    {
        let mut readings = Vec::new();
    
        let re = Regex::new(r"^(?P<prefix>[0-9]+)?\s*(?P<book>[A-Za-z ]+) (?P<chapter>\d+)(?::(?P<start>\d+)-(?P<end>\d+)|-(?P<chapter_end>\d+))?")
            .map_err(|_| ParseError::InvalidFormat)?;
    
        for line in text.lines() 
        {
            if let Some(caps) = re.captures(line.trim()) 
            {
                let prefix: Option<u32> = match caps.name("prefix") 
                {
                    Some(p) => Some(p.as_str().parse().map_err(|_| ParseError::InvalidNumber)?),
                    None => None,
                };

                let book = caps["book"].to_string();
                let chapter_start: u32 = caps["chapter"].parse().map_err(|_| ParseError::InvalidNumber)?;
                let chapter_start = chapter_start - 1;
    
                if let Some(start) = caps.name("start") 
                {
                    let start: u32 = start.as_str().parse().map_err(|_| ParseError::InvalidNumber)?;
                    let end: u32 = caps["end"].parse().map_err(|_| ParseError::InvalidNumber)?;
                    let range = VerseRange { start: start - 1, end: end - 1 };
                    readings.push(Reading {
                        prefix,
                        book,
                        chapter: chapter_start,
                        range: Some(range),
                    });
                } 
                else if let Some(chapter_end) = caps.name("chapter_end") 
                {
                    let chapter_end: u32 = chapter_end.as_str().parse().map_err(|_| ParseError::InvalidNumber)?;
                    let chapter_end = chapter_end - 1;
                    for i in chapter_start..=chapter_end
                    {
                        readings.push(Reading {
                            prefix,
                            book: book.clone(),
                            chapter: i,
                            range: None
                        });
                    }
                }
                else 
                {
                    readings.push(Reading {
                        prefix,
                        book,
                        chapter: chapter_start,
                        range: None
                    });
                }
            } 
            else 
            {
                return Err(ParseError::InvalidFormat);
            }
        }
    
        Ok(readings)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectedReading 
{
    #[serde(rename = "0")]
    RobertRoberts,

    #[serde(rename = "2")]
    Chronological,

    #[serde(rename = "1")]
    Proverbs,
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_reading(state: State<'_, ReadingsDatabase>, month: u32, day: u32, selected_reading: &str) -> Vec<Reading>
{
    state.get_readings(month, day, serde_json::from_str(selected_reading).unwrap())
}