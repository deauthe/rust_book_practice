fn main() {
    let mut ghoul : Option<Box<String>> = Some(Box::new(String::from("hello ji")));
    print!("ghoul:{:?},  ghoul.unwrap():{} \n",ghoul,ghoul.as_ref().unwrap());

    let refe = &mut ghoul;
    
    print!("*refe {:?}, refe: {:?} \n ",*refe, refe);

    let refe2 = &mut *refe;
    //derefs the &mut and then adds the &mut again, weird syntax,
    //but is used in the linked_list implementation
    print!("refe2:{:?}",refe2);


}
