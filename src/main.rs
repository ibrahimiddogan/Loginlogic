fn main() {
    newMember("İbrahim".to_string(), "Doğan".to_string(), 
    12322323, "ibrahimgmail.com".to_string(),
     "1221213".to_string());

     newPosting("hata.jpg".to_string(), "bugün hava çok güzel ".to_string());
     
}


struct Member {

    name:String,
    lastname:String,
    phone:u64,
    email:String,
    password:String,
}


fn newMember(name:String,lastname:String,phone:u64,
    email:String,
    password:String,) {
        if !email.contains("@") {

            println!("Böyle bir mail formatı yok")
            
        }else {
            let member = Member{
                name:name,
                lastname:lastname,
                phone:phone,
                email:email,
                password:password
            };

            println!("Kaydınız başarılı oldu")
            
        }

    
}

struct Posting{
    image:String,
    description:String,
}


fn newPosting(image:String,description:String) {

    let newpost = Posting{
        image:image,
        description:description,
    };
    println!("Başarılı şekilde hikayeniz paylaşıldı ")

    
}