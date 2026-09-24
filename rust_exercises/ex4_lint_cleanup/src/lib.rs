//! Channel name parsing. The logic works. Everything else about this file is
//! wrong on purpose. See EXERCISE.md.

/// Splits a channel name like "ai_12" into its prefix and numeric id.
pub fn parse_channel_name(name:&String)->Option<(String,u16)>{
  if name.len()==0{
    return None;
  }
  let parts:Vec<&str>=name.split("_").collect();
  if parts.len()==2{
      if let Ok(id)=parts[1].parse::<u16>(){
          let prefix=parts[0].to_string();
          return Some((prefix,id));
      }
  }
  return None;
}

/// Returns the ids of every name that parsed successfully.
pub fn collect_ids(names:&Vec<String>)->Vec<u16>{
    let mut ids=Vec::new();
    for i in 0..names.len(){
        let name=names[i].clone();
        match parse_channel_name(&name){
            Some(parsed)=>{ids.push(parsed.1);},
            None=>{}
        }
    }
    return ids;
}

/// True if the name refers to an analog input.
pub fn is_analog_input(name:&String)->bool{
    let parsed=parse_channel_name(name);
    match parsed.is_some(){
        true=>{
            let value=parsed.unwrap();
            if value.0=="ai"{return true;}else{return false;}
        },
        false=>return false
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_parse_channel_name_returns_prefix_and_id(){
        let name=String::from("ai_12");
        let parsed=parse_channel_name(&name);
        assert_eq!(parsed,Some((String::from("ai"),12)));
    }

    #[rstest]
    fn test_collect_ids_skips_unparseable_names(){
        let names=vec![String::from("ai_1"),String::from("garbage"),String::from("do_7")];
        let ids=collect_ids(&names);
        assert_eq!(ids,vec![1,7]);
    }

    #[rstest]
    fn test_is_analog_input(){
        assert_eq!(is_analog_input(&String::from("ai_3")),true);
        assert_eq!(is_analog_input(&String::from("do_3")),false);
    }
}
