use nom::{
  branch::alt,
  bytes::complete::tag,
  multi::many0,
  IResult,
  sequence::tuple,
};
  
use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::scanners::texto::*;
use crate::parser::asginacion::*;
use crate::parser::funcion_esp::*;
use crate::parser::llama_func::*;
use crate::parser::repeticion::*;
use crate::parser::decision::*;
use crate::parser::comentario::*;


pub fn estatuto(input: &str) -> IResult<&str, &str> {
  alt((asginacion, funcion_esp, llama_func, repeticion, decision, comentario))(input)
}
  