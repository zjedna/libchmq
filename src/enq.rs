

	
#![allow(dead_code)]
#![allow(unused_imports)]

use rand;
use rand::Rng;
use std::string::String;
use std::vec::Vec;
use std::f64;
use std::f64::INFINITY;

use Compound;
use Reaction;
use Equilibrium::{self,Keq,DeltaH};

use dis;
use dis_u;
use ff;
use abs;
use absf64;
use parse_compound_json;
use form_root;

use AB_Z;
use TEN;
use R;
use GAS;
use LIQ;
use SOL;
use AQU;



pub const TITLE:&'static str="

  Med Chem Quiz V.1.1.1
  (Chmq)
  2018-10-28              
";


pub const WARNING:&'static str=" \
This set of exercises is designed to drill the user in chemical calculations. \
Primarily it is meant to drill the user in calculations from the \"Calculations in Medical Chemistry\" module. \
However it has three very serious limitations:

1) This is a drill. \
It should increase the user's proficiency in calculations, \
but it is unlikely to help you understand the topic. 

2) It is important to try a wide variety of calculations to become proficient in \"juggling with numbers\", \
but the questions here follow a limited number of fixed patterns. \
It is therefore important to challenge yourself with questions from other sources.

3) The questions follow a fixed pattern of wording. \
This means that these exercises do not develop problem solving skills to the extent \
which would be desirable for working with calculations in \"real world\" scenarios. \
It can also lead to \"language shock\" when users encounter familiar questions with unfamiliar wording!

You have been warned!
";


pub const ABOUT:&'static str=
"Name:     Med Chem Quiz                      
Version:  1.11                          
Created:  2018-11 (2017-11)                            
Author:   Aleksey Zholobenko
For:      Calculations in Medical Chemistry, 
          First Year Module,                 
          Faculty of Medicine, Olomouc.
          (...And general use.)     
Licence:  CC BY-SA";


//function for automatically retrieving help for a question based on compounds present.
pub fn helper(query: &str, library:&Vec<Compound>)-> (String,String) {
	
	//let problem=query.to_owned();
	let mut solution:String = String::new();
	let mut mini_help:String = String::new();
	for x in library.iter(){
		if query.contains(&x.name[0]){
			solution.push_str(&format!("{}\n\n",form_chem(x)));
			mini_help.push_str(&format!("{}\n",mini_form_chem(x)));
		}else{};
	};
	let solution = if solution.len()==0 {"No help is available".to_owned()}else{solution.trim().to_owned()};
	let mini_help = if mini_help.len()==0 {"No data sheet.".to_owned()}else{mini_help.trim().to_owned()};
	(solution,mini_help)
}

#[no_mangle]
#[allow(unused_variables)]
//function to display compound. in english.
pub fn form_chem(q:&Compound)->String{
	
	let c_type:&str= if (q.use_weak==false) & (q.salt==true){"salt of strong acid and strong base"
					 }else if (q.use_weak==true) & (q.salt==true) & (q.pka[0].0<7.0){"salt of weak acid and strong base"
					 }else if (q.use_weak==true) & (q.salt==true) & (q.pka[0].0>7.0){"salt of strong acid and weak base"
					 }else if (q.use_weak==false) & (q.salt==false) & (q.pka[0].0>7.0){"strong base"
					 }else if (q.use_weak==false) & (q.salt==false) & (q.pka[0].0<7.0){"strong acid"
					 }else if (q.use_weak==true) & (q.salt==false) & (q.pka[0].0<7.0){"weak acid"
					 }else if (q.use_weak==true) & (q.salt==false) & (q.pka[0].0>7.0){"weak base"
					 }else if (q.med.0==true)& (q.med.2>0.0){"It's a medically useful compound"
					 }else{"It's a chemical compound of some kind."};
	let mut output:Vec<String>=Vec::new();				 
	output.push(format!("{}       {}",
		"Name:",
		format!("{}",q.name[0])));
	output.push(format!("{}    {}",
		"Formula:",q.formula[0]));
	output.push(format!("{} {} (g/mol)",
		"Molar mass:",q.mmass));
		
	//Show multiple pKas correctly.
	let pka=if q.pka[0].0==7.0{
		"-".to_owned()
	}else if q.pka.len()==0 {
		format!("{}",q.pka[0].0)
	}else{
		let mut pka_temp:Vec<String> = Vec::new();
		for x in q.pka.iter(){pka_temp.push(format!("{}",x.0))};
		pka_temp.join(", ")
	};
	output.push(format!("{}        {}",
		"pKa:",pka));
	output.push(format!("{}       {}",
		"Type:",c_type));
	if q.solubility==f64::INFINITY{
	output.push(format!("{} Miscible with water at any ratio.",
		"Solubility:"));
	}else{
	output.push(format!("{} {}g/100mL",
		"Solubility:",dis(q.solubility)));
	};
	if q.solutes.len()<2{
		output.push(format!("{} does not dissociate in aqueous solutions.",q.name[0]));
	}else{
		let partial=if q.use_weak==true{" partially"}else{""};
		output.push(format!("{}",format!("In aqueous solutions{} dissociates to:",partial)));
		for x in q.solutes.iter(){
			let p_m= if x.2<0 {format!("({}-)",abs(x.2))
					}else if x.2>0 {format!("({}+)",abs(x.2))
					}else{format!("")};
			output.push(format!("{} x {} {}",x.0,x.1,p_m));
		}
	};
	output.join("\n")
}

//formchem for a simple, less helpful minitable.
pub fn mini_form_chem(q:&Compound)->String{
	
	let mut output = Vec::new();
	
	
	output.push(format!("{} {}",
		"Name:",
		format!("{}",q.name[0]))
	);
	output.push(format!("{} {} (g/mol)",
		"Molar mass:",q.mmass)
	);
		
	if q.solubility==f64::INFINITY{
		output.push(format!("{} Miscible with water at any ratio.",
			"Solubility:")
		);
	}else{
		output.push(format!("{} {}g/100mL",
			"Solubility:",dis(q.solubility))
		);
	}
		
	//Show multiple pKas correctly.
	let pka=if q.pka[0].0==7.0{
		"-".to_owned()
	}else if q.pka.len()==0 {
		format!("{}",q.pka[0].0)
	}else{
		let mut pka_temp:Vec<String> = Vec::new();
		for x in q.pka.iter(){pka_temp.push(format!("{}",x.0))};
		pka_temp.join(", ")
	};
	
	output.push(format!("{} {}",
		"pKa:",pka));
		
	output.join(", ")
}

//MOLES QUESTIONS
//MOLES QUESTIONS
//MOLES QUESTIONS
//MOLES QUESTIONS

pub fn q_1_0(compounds:&Vec<Compound>)->(String,String) {
//Question of type n=m/Mr.
	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate mass of compound in question.
	let m:f64=(rand::thread_rng().gen_range(1,2001) as f64)/100.0;
	
	//generate answer.
	let answer=m/c.mmass;
	
	//println!("m: {}",m);
	//println!("answer: {}",answer);
	
	let question=format!("How many moles of {} are there in {} of the compound?",
		c.name[0],
		format!("{}g",dis(m)));
		
	let answer=format!("Moles of {} can be calculated by dividing the mass of {} by its molar mass.\n\n {}",
		c.name[0],
		c.name[0],
		format!(" Answer = {}mol",dis(answer))
		);
	(question,answer)	
}


pub fn q_1_1(compounds:&Vec<Compound>)->(String,String){
//Question of type m=n*Mr.
	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate moles of compound in question.
	let n:f64=(rand::thread_rng().gen_range(1,301) as f64)/100.0;
	
	//generate answer.
	let answer=n*c.mmass;
	
	//println!("n: {}",n);
	//println!("answer: {}",answer);
	
	let question=format!("How many grams of {} are there in {} of the compound?",
		c.name[0],
		format!("{}mol",dis(n))
	);

	let answer=format!("Mass of {} can be calculated by multiplying moles of {} by its molar mass.\n\n{}\n",
		c.name[0],
		c.name[0],
		format!(" Answer = {}g",dis(answer))
	);
	(question,answer)
}


pub fn q_1_2(compounds:&Vec<Compound>)->(String,String){
//Question of type Molarity=n/Vol.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate mass of compound in question.
	let mut m:f64=(rand::thread_rng().gen_range(10,2001) as f64)/100.0;

	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(20,501) as f64)/100.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if m/v_litre/10.0>c.solubility{m=m/10.0}else{silly=false}
	};
	
	//generate answer.
	let answer=m/c.mmass/v_litre;
	
	//println!("m: {}",m);
	//println!("v_litre: {}",v_litre);
	//println!("answer: {}",answer);
	
	let question=format!("What is the molarity of a solution containing {}g of {} in a volume of {} litres?",
		dis(m),
		c.name[0],
		v_litre);
	
	let answer=format!("mols of {}: {}\n\
	Molarity of {} can be calculated by dividing number of moles by its volume.\
	\n\n Answer = {}mol/L",
		c.name[0],
		m/c.mmass,
		c.name[0],
		dis(answer));
	(question,answer)
}


pub fn q_1_2c(compounds:&Vec<Compound>)->(String,String){
//Question of type Molarity=n/Vol.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate mass of compound in question.
	let mut m:f64=(rand::thread_rng().gen_range(10,2001) as f64)/100.0;

	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(20,501) as f64)/100.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if m/v_litre/10.0>c.solubility{m=m/10.0}else{silly=false}
	};
	
	//generate answer.
	let answer=m/c.mmass/v_litre;
	
	//println!("m: {}",m);
	//println!("v_litre: {}",v_litre);
	//println!("answer: {}",answer);
	
	let question=format!("What is the molarity of a solution containing {}g of {} in a volume of {} litres?",
		dis(m),
		c.name[0],
		v_litre);
	
	let answer=format!("mols of {}: {}\n\
	Molarity of {} can be calculated by dividing number of moles by its volume.\
	\n\n Answer = {}mol/L",
		c.name[0],
		m/c.mmass,
		c.name[0],
		dis(answer));
	(question,answer)
}


pub fn q_1_2b(compounds:&Vec<Compound>)->(String,String){ //INCOMPLETE
//Question of type n=Molarity*Vol.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(20,501) as f64)/100.0;	
	
	//generate mass of compound in question.
	let mut conc:f64=(rand::thread_rng().gen_range(10,2001) as f64)/500.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if conc*c.mmass/10.0>c.solubility{conc=conc/10.0}else{silly=false}
	};
	
	//generate answer.
	let answer=conc*v_litre;
	
	//println!("c: {}",conc);
	//println!("v_litre: {}",v_litre);
	//println!("answer: {}",answer);
	
	let question=format!("A {} solution with a molarity of {}mol/L has a volume of {} litres. How many moles of {} does it contain?",
		c.name[0],
		dis(conc),
		v_litre,
		c.name[0]);
	
	let answer=format!("Moles of {} can be calculated by multiplying volume of solution by molarity.\
	\n\n Answer = {}mol",
		c.name[0],
		dis(answer));
	(question,answer)
}


pub fn q_1_3(compounds:&Vec<Compound>)->(String,String){
//Question of type m=V*C*Mr
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;

	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate mass of compound in question.
	let mut conc:f64=(rand::thread_rng().gen_range(10,2001) as f64)/1000.0;

	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(20,501) as f64)/100.0;	
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if conc*c.mmass/10.0>c.solubility{conc=conc/10.0}else{silly=false}
	};
	
	//generate answer.
	let answer=conc*c.mmass*v_litre;
	
	//println!("c: {}",conc);
	//println!("v_litre: {}",v_litre);
	//println!("answer: {}",answer);
	
	let question=format!("What is the mass of {} in {} litres of solution with a concentration of {}mol/L?",
		c.name[0],
		v_litre,
		dis(conc)
	);
	
	let answer=format!("mols of {}: {}\n\n{}\n",
		c.name[0],conc*v_litre,(format!(" Answer = {}g",dis(answer))));
	(question,answer)	
}


pub fn q_1_4(compounds:&Vec<Compound>)->(String,String){
//C1V1=C2V2 type 1 (C1V1=C2V2 questions)

	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate mass of compound in question.
	let mut c_1:f64=(rand::thread_rng().gen_range(10,2001) as f64)/1000.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if c_1*c.mmass/10.0>c.solubility{c_1=c_1/10.0}else{silly=false}
	};
	
	//generate df
	let df:f64=	rand::thread_rng().gen_range(2,20) as f64;
	
	//generate volume.
	let v_1:f64=(rand::thread_rng().gen_range(5,501) as f64)/250.0;
	let v_2:f64=v_1*df;
	let c_2:f64=c_1/df;
	
	//println!("v_1: {}",v_1);
	//println!("v_2: {}",v_2);
	//println!("c_1: {}",c_1);
	//println!("c_2: {}",c_2);
	
	//diluting or concentrating?
	let diluting= if 5>rand::thread_rng().gen_range(0,10) {true}else{false};
	let find_c= if 5>rand::thread_rng().gen_range(0,10) {true}else{false};
	
	let question:String = if (diluting==true) & (find_c==true) {
		format!("A {} solution has a concentration of {}mol/L and a volume of {}L. \
		It is then diluted to a final volume of {}L. \nWhat is the concentration of the final dilution?",
		c.name[0],dis(c_1),dis(v_1),dis(v_2))
	}else if (diluting==true) & (find_c==false) {
		format!("A {} solution has a concentration of {}mol/L and a volume of {}L. \
		It is then diluted to a final concentration of {}mol/L. \nWhat is the volume of the final dilution?",
		c.name[0],dis(c_1),dis(v_1),dis(c_2))
	}else if (diluting==false) & (find_c==true) {
		format!("A diluted solution of {} has a concentration of {}mol/L and a volume of {}L. \
		The initial solution had a volume of {}L. \nWhat was the concentration of the initial solution?",
		c.name[0],dis(c_2),dis(v_2),dis(v_1))
	}else{
		format!("A diluted solution of {} has a concentration of {}mol/L and a volume of {}L. \
		The inital solution had a concentration of {}mol/L. \nWhat was the volume of the initial solution?",
		c.name[0],dis(c_2),dis(v_2),dis(c_1))
	};
	
	let answer_a = format!("Ci x Vi = Cf x Vf");
	
	let answer_b = if (diluting==true) & (find_c==true) {
		format!("{}\n",format!(" Answer (Cf) = {}mol/L\n",dis(c_2)))
	}else if (diluting==true) & (find_c==false) {
		format!("{}\n",format!(" Answer (Vf) = {}L\n",dis(v_2)))
	}else if (diluting==false) & (find_c==true) {
		format!("{}\n",format!(" Answer (Ci) = {}mol/L\n",dis(c_1)))
	}else{
		format!("{}\n",format!(" Answer (Vi) = {}L\n",dis(v_1)))
	};
	let answer=format!("{}\n\n{}",answer_a,answer_b);	
	(question,answer)
}


pub fn q_1_4b(compounds:&Vec<Compound>)->(String,String){
//Dilution factor type based on C1V1=C2V2.

	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate mass of compound in question.
	let mut c_1:f64=(rand::thread_rng().gen_range(10,2001) as f64)/1000.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if c_1*c.mmass/10.0>c.solubility{c_1=c_1/10.0}else{silly=false}
	};
	
	//generate df
	let df:f64=	rand::thread_rng().gen_range(2,20) as f64;
	
	//generate volume.
	let v_1:f64=(rand::thread_rng().gen_range(5,501) as f64)/250.0;
	let v_2:f64=v_1*df;
	let c_2:f64=c_1/df;
	
	//diluting or concentrating?
	let diluting= if 5>rand::thread_rng().gen_range(0,10) {true}else{false};
	let find_c= if 5>rand::thread_rng().gen_range(0,10) {true}else{false};
	
	//x-fold or 1:(x-1) ?
	let fold= if 5>rand::thread_rng().gen_range(0,10) {true}else{false};
	
	let fold_or_to= if fold==true {format!("{}-fold",df)}else{format!("1:{}",df-1.0)};
	
	//PRINT QUESTION
	let question:String = if (diluting==true) & (find_c==true) {
		format!("A {} solution has a concentration of {}mol/L and a volume of {}L. \
		It is then diluted {}. \nWhat is the concentration of the final dilution?",
		c.name[0],dis(c_1),dis(v_1),fold_or_to)
	}else if (diluting==true) & (find_c==false) {
		format!("A {} solution has a concentration of {}mol/L and a volume of {}L. \
		It is then diluted {}. \
		\nWhat is the volume of the final dilution? \
		\nWhat was the volume of diluent added?",
		c.name[0],dis(c_1),dis(v_1),fold_or_to)
	}else if (diluting==false) & (find_c==true) {
		format!("A {} solution was diluted {}. The diluted solution has a concentration of {}mol/L and a volume of {}L. \
		\nWhat was the concentration of the initial solution?",
		c.name[0],fold_or_to,dis(c_2),dis(v_2))
	}else{
		format!("A {} solution was diluted {}. The diluted solution has a concentration of {}mol/L and a volume of {}L. \
		\nWhat was the volume of the initial solution? \
		\nWhat was the volume of diluent added?",
		c.name[0],fold_or_to,dis(c_2),dis(v_2))
	};
	
	let answer_a = format!("Ci x Vi = Cf x Vf");
	let answer_b = format!("Df = Vf/Vi = Ci/Cf.");
	let answer_c = if !fold {format!("Solution diluted 1:(Df-1)")}else{format!("")};
	let answer_cii = format!("Df = {}",df);
	
	let answer_d = if (diluting==true) & (find_c==true) {
		format!("{}\n",format!(" Answer (Cf) = {}mol/L\n",dis(c_2)))
	}else if (diluting==true) & (find_c==false) {
		format!("{}\n(Volume diluent = {}L)\n",format!(" Answer (Vf) = {}L",dis(v_2)),dis(v_2-v_1))
	}else if (diluting==false) & (find_c==true) {
		format!("{}\n",format!(" Answer (Ci) = {}mol/L\n",dis(c_1)))
	}else{
		format!("{}\n(Volume diluent = {}L)\n",format!(" Answer (Vi) = {}L",dis(v_1)),dis(v_2-v_1))
	};	
	let answer=format!("{}\n{}\n{}\n{}\n\n{}\n",
		answer_a,
		answer_b,
		answer_c,
		answer_cii,
		answer_d);	
	(question,answer)
}


pub fn q_1_4c(compounds:&Vec<Compound>)->(String,String){
//C1V1=C2V2 type 1 (C1V1=C2V2 questions) with mass

	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate mass of compound in question.
	let mut mc_1:f64=(rand::thread_rng().gen_range(10,10001) as f64)/1000.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if mc_1/10.0>c.solubility{mc_1=mc_1/10.0}else{silly=false}
	};
	
	//generate df
	let df:f64=	rand::thread_rng().gen_range(2,20) as f64;
	
	//generate volume.
	let v_1:f64=(rand::thread_rng().gen_range(5,501) as f64)/250.0;
	let v_2:f64=v_1*df;
	let mc_2:f64=mc_1/df;
	
	//println!("v_1: {}",v_1);
	//println!("v_2: {}",v_2);
	//println!("c_1: {}",c_1);
	//println!("c_2: {}",c_2);
	
	//diluting or concentrating?
	let diluting= if 5>rand::thread_rng().gen_range(0,10) {true}else{false};
	let find_c= if 5>rand::thread_rng().gen_range(0,10) {true}else{false};
	
	let question:String = if (diluting==true) & (find_c==true) {
		format!("A {} solution has a concentration of {}g/L and a volume of {}L. \
		It is then diluted to a final volume of {}L. \nWhat is the concentration of the final dilution?",
		c.name[0],dis(mc_1),dis(v_1),dis(v_2))
	}else if (diluting==true) & (find_c==false) {
		format!("A {} solution has a concentration of {}g/L and a volume of {}L. \
		It is then diluted to a final concentration of {}g/L. \nWhat is the volume of the final dilution?",
		c.name[0],dis(mc_1),dis(v_1),dis(mc_2))
	}else if (diluting==false) & (find_c==true) {
		format!("A diluted solution of {} has a concentration of {}g/L and a volume of {}L. \
		The initial solution had a volume of {}L. \nWhat was the concentration of the initial solution?",
		c.name[0],dis(mc_2),dis(v_2),dis(v_1))
	}else{
		format!("A diluted solution of {} has a concentration of {}g/L and a volume of {}L. \
		The inital solution had a concentration of {}g/L. \nWhat was the volume of the initial solution?",
		c.name[0],dis(mc_2),dis(v_2),dis(mc_1))
	};
	
	let answer_a = format!("Ci x Vi = Cf x Vf");
	
	let answer_b = if (diluting==true) & (find_c==true) {
		format!("{}\n",format!(" Answer (Cf) = {}g/L\n",dis(mc_2)))
	}else if (diluting==true) & (find_c==false) {
		format!("{}\n",format!(" Answer (Vf) = {}L\n",dis(v_2)))
	}else if (diluting==false) & (find_c==true) {
		format!("{}\n",format!(" Answer (Ci) = {}g/L\n",dis(mc_1)))
	}else{
		format!("{}\n",format!(" Answer (Vi) = {}L\n",dis(v_1)))
	};
	let answer=format!("{}\n\n{}",answer_a,answer_b);	
	(question,answer)
}


pub fn q_1_4d(compounds:&Vec<Compound>)->(String,String){
//Dilution factor type based on C1V1=C2V2. (mass type)

	//generate compound
	let c_len=compounds.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&compounds[indx];
	
	//generate mass of compound in question.
	let mut mc_1:f64=(rand::thread_rng().gen_range(10,10001) as f64)/1000.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if mc_1/10.0>c.solubility{mc_1=mc_1/10.0}else{silly=false}
	};
	
	//generate df
	let df:f64=	rand::thread_rng().gen_range(2,20) as f64;
	
	//generate volume.
	let v_1:f64=(rand::thread_rng().gen_range(5,501) as f64)/250.0;
	let v_2:f64=v_1*df;
	let mc_2:f64=mc_1/df;
	
	//diluting or concentrating?
	let diluting= if 5>rand::thread_rng().gen_range(0,10) {true}else{false};
	let find_c= if 5>rand::thread_rng().gen_range(0,10) {true}else{false};
	
	//x-fold or 1:(x-1) ?
	let fold= if 5>rand::thread_rng().gen_range(0,10) {true}else{false};
	
	let fold_or_to= if fold==true {format!("{}-fold",df)}else{format!("1:{}",df-1.0)};
	
	//PRINT QUESTION
	let question:String = if (diluting==true) & (find_c==true) {
		format!("A {} solution has a concentration of {}g/L and a volume of {}L. \
		It is then diluted {}. \nWhat is the concentration of the final dilution?",
		c.name[0],dis(mc_1),dis(v_1),fold_or_to)
	}else if (diluting==true) & (find_c==false) {
		format!("A {} solution has a concentration of {}g/L and a volume of {}L. \
		It is then diluted {}. \
		\nWhat is the volume of the final dilution? \
		\nWhat was the volume of diluent added?",
		c.name[0],dis(mc_1),dis(v_1),fold_or_to)
	}else if (diluting==false) & (find_c==true) {
		format!("A {} solution was diluted {}. The diluted solution has a concentration of {}g/L and a volume of {}L. \
		\nWhat was the concentration of the initial solution?",
		c.name[0],fold_or_to,dis(mc_2),dis(v_2))
	}else{
		format!("A {} solution was diluted {}. The diluted solution has a concentration of {}g/L and a volume of {}L. \
		\nWhat was the volume of the initial solution? \
		\nWhat was the volume of diluent added?",
		c.name[0],fold_or_to,dis(mc_2),dis(v_2))
	};
	
	let answer_a = format!("Ci x Vi = Cf x Vf");
	let answer_b = format!("Df = Vf/Vi = Ci/Cf.");
	let answer_c = if !fold {format!("Solution diluted 1:(Df-1)")}else{format!("")};
	let answer_cii = format!("Df = {}",df);
	
	let answer_d = if (diluting==true) & (find_c==true) {
		format!("{}\n",format!(" Answer (Cf) = {}g/L\n",dis(mc_2)))
	}else if (diluting==true) & (find_c==false) {
		format!("{}\n(Volume diluent = {}L)\n",format!(" Answer (Vf) = {}L",dis(v_2)),dis(v_2-v_1))
	}else if (diluting==false) & (find_c==true) {
		format!("{}\n",format!(" Answer (Ci) = {}g/L\n",dis(mc_1)))
	}else{
		format!("{}\n(Volume diluent = {}L)\n",format!(" Answer (Vi) = {}L",dis(v_1)),dis(v_2-v_1))
	};	
	let answer=format!("{}\n{}\n{}\n{}\n\n{}\n",
		answer_a,
		answer_b,
		answer_c,
		answer_cii,
		answer_d);	
	(question,answer)
}


//OSMOLARITY QUESTION TYPES.
//OSMOLARITY QUESTION TYPES.
//OSMOLARITY QUESTION TYPES.
//OSMOLARITY QUESTION TYPES.
//OSMOLARITY QUESTION TYPES.
//OSMOLARITY QUESTION TYPES.


pub fn q_2_0(compounds:&Vec<Compound>)->(String,String){
//Question of type Osmoles=sum(Cs).
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;

	//generate compound
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.use_weak==false) || (x.salt==true){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	
	//generate mass of compound in question.
	let m:f64=(rand::thread_rng().gen_range(1,2001) as f64)/100.0;
	
	//generate answer.
	let mut solutes=0;
	for x in c.solutes.iter(){
		solutes+=x.0
	};
	let answer=m/c.mmass*(solutes as f64);
	
	let question = format!("How many osmoles of {} are there in {}g of the compound?",
			c.name[0],
			dis(m));
	
	let answer = format!("Osmoles of {} can be calculated by adding up the number of moles of each solute produced when the compound dissociates.\n\n {}\n",
		c.name[0],
		format!(" Answer = {}Osmol",dis(answer))
	);
	(question,answer)
}


pub fn q_2_1(compounds:&Vec<Compound>)->(String,String){
//Question of type m=osmoles*Mr.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	//generate compound
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.use_weak==false) || (x.salt==true){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	
	//generate moles of compound in question.
	let n:f64=(rand::thread_rng().gen_range(1,301) as f64)/100.0;
	
	//generate answer.
	let mut solutes=0;
	for x in c.solutes.iter(){
		solutes+=x.0
	};
	let answer=n*c.mmass/(solutes as f64);
	
	let question = format!("How many grams of {} are there in {}Osmol of the compound?",
		c.name[0],dis(n));
	
	let answer = format!("Mass of {} can be calculated by multiplying moles of {} by its molar mass.\n\n {}\n",
		c.name[0],
		c.name[0],
		format!(" Answer = {}g",dis(answer))
	);
	(question,answer)	
}




pub fn q_2_2(compounds:&Vec<Compound>)->(String,String){
//Question of type Osmolarity=sum(Cs)/Vol.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;

	//generate compound
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.use_weak==false) || (x.salt==true){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	
	
	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(20,501) as f64)/100.0;	
	
	//generate mass of compound in question.
	let mut m:f64=(rand::thread_rng().gen_range(10,2001) as f64)/100.0;
	
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if m/v_litre/10.0>c.solubility{m=m/10.0}else{silly=false};	
	};
	
	//generate answer.
	let mut solutes=0;
	for x in c.solutes.iter(){
		solutes+=x.0
	};
	
	//generate answer.
	let sf64=solutes as f64;
	let answer=m/c.mmass*sf64/v_litre;
	
	let question = format!("What is the osmolarity of a solution containing {}g of {} in a volume of {} litres?",
		dis(m),
		c.name[0],
		v_litre
	);
	
	let answer_a=format!("Moles of {} (mol) = {}",c.name[0],m/c.mmass);
	let answer_b=format!("Osmoles of {} (osmol) = {}",c.name[0],m/c.mmass*sf64);
	let answer_c=format!("{}\n",format!("Answer= {}Osmol/L",dis(answer)));
	let answer = format!("{}\n{}\n\n {}\n",answer_a,answer_b,answer_c);
	(question,answer)
}


pub fn q_2_3(compounds:&Vec<Compound>)->(String,String){
//Question of type mass=Osmolarity/(n_solutes)*Volume*Mr.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	
	//generate compound
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.use_weak==false) || (x.salt==true){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	
	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(20,501) as f64)/100.0;	
	
	//generate mass of compound in question.
	let mut osm:f64=(rand::thread_rng().gen_range(10,2001) as f64)/100.0;
	
	//generate answer.
	let mut solutes=0;
	for x in c.solutes.iter(){
		solutes+=x.0
	};
	let sf64=solutes as f64;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if osm/sf64*c.mmass/10.0>c.solubility{osm=osm/10.0}else{silly=false};
	};
	
	//generate answer.
	let answer=osm*c.mmass*v_litre/sf64;
	
	let question = format!("What is the mass of {} in {} litres of {} solution with an osmolarity of {}Osmol/L?",
		c.name[0],
		v_litre,
		c.name[0],
		dis(osm)
	);
	
	let ans_a=format!("Osmoles of {} (osmol) = {}",c.name[0],osm*v_litre);
	let ans_b=format!("Moles of {} (mol) = {}",c.name[0],osm*v_litre/sf64);
	let ans_c=format!("{}\n",format!("Answer = {}g",dis(answer)));
	let answer = format!("\n{}\n{}\n\n {}\n",ans_a,ans_b,ans_c);
	(question,answer)	
}


pub fn q_2_4(compounds:&Vec<Compound>)->(String,String){
	//Question of type Osmotic Pressure=1000*R*T*sum(Cs).
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	
	//generate compound
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if ((x.use_weak==false) || (x.salt==true))
		& ((x.solubility/x.mmass)>0.000001){v_valid.push(x)
		}else{}
	};
	
	//generate compound vector
	let c_len=v_valid.len();
	let mut comp_vec:Vec<&Compound>=Vec::new();
	let mut c_ind_vec:Vec<usize>=Vec::new();
	for i in 0..c_len{c_ind_vec.push(i)};
	//number of compounds
	let no_comps=rand::thread_rng().gen_range(2,5);
	
	//pick compounds
	for _ in 0..no_comps{
		let indx_a=rand::thread_rng().gen_range(0,c_ind_vec.len());
		let index=c_ind_vec.remove(indx_a);
		comp_vec.push(v_valid[index])
	};
	
	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(20,501) as f64)/100.0;
	
	//generate mass of compounds in question.
	let mut m_comps:Vec<f64>=Vec::new();
	for i in 0..no_comps{
		let mut m:f64=(rand::thread_rng().gen_range(50,2001) as f64)/100.0;
		//Check solubility
		if m/v_litre/10.0>comp_vec[i].solubility {m=10.0*comp_vec[i].solubility*v_litre}else{};	
		m_comps.push(m)
	};
	
	//temperature generation.
	let temp_c:f64=rand::thread_rng().gen_range(1,50) as f64;
	let temp_k:f64=temp_c-AB_Z;
	
	//generate answer.
	let mut osmoles=0.0;
	for i in 0..comp_vec.len(){
		let mut solutes=0;
		for x in comp_vec[i].solutes.iter(){
			solutes+=x.0
		};
		osmoles+=m_comps[i]/comp_vec[i].mmass*(solutes as f64)
	};

	let answer=R*temp_k*osmoles/v_litre;

	//Generate question text.
	let question_a=format!("What is the osmotic pressure of a solution containing:");
	let mut question_b:Vec<String>=Vec::new();
	for i in 0..no_comps{
		question_b.push(format!("{}g of {}",
			dis(m_comps[i]),
			comp_vec[i].name[0]
		))
	};
	let question_b=question_b.join("\n");
	let question_c=format!("In a total volume of {} litres at {} degrees Celsius?",v_litre,temp_c);
	
	let question = format!("{}\n{}\n{}\n",question_a,question_b,question_c);
	
	//Generate answer text.
	let ans_a=format!("Temperature (Kelvin) = {}",temp_k);
	let ans_b=format!("Osmolarity (osmol/L) = {}",osmoles/v_litre);
	let ans_c=format!("{}",format!("Answer = {} KPa",&ff(4,answer)));
	let answer = format!("{}\n{}\n\n {}\n",ans_a,ans_b,ans_c);
	(question,answer)
}


pub fn q_2_4s(compounds:&Vec<Compound>)->(String,String){
	//Question of INVERSE type Osmotic Pressure=1000*R*T*sum(Cs).
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	
	//Find mass of of first compound in the list.
	//generate compound
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if ((x.use_weak==false) || (x.salt==true))
		& ((x.solubility/x.mmass)>0.000001){v_valid.push(x)
		}else{}
	};

	//generate compound vector
	let c_len=v_valid.len();
	let mut comp_vec:Vec<&Compound>=Vec::new();
	let mut c_ind_vec:Vec<usize>=Vec::new();
	for i in 0..c_len{c_ind_vec.push(i)};
	//number of compounds
	let no_comps=rand::thread_rng().gen_range(2,5);
	
	//pick compounds
	for _ in 0..no_comps{
		let indx_a=rand::thread_rng().gen_range(0,c_ind_vec.len());
		let index=c_ind_vec.remove(indx_a);
		comp_vec.push(v_valid[index])
	};
		
	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(20,501) as f64)/100.0;
	
	//generate mass of compounds in question.
	let mut m_comps:Vec<f64>=Vec::new();
	for i in 0..no_comps{
		let mut m:f64=(rand::thread_rng().gen_range(50,2001) as f64)/100.0;
		//Check solubility
		if m/v_litre/10.0>comp_vec[i].solubility {m=10.0*comp_vec[i].solubility*v_litre}else{};	
		m_comps.push(m)
	};
	
	//temperature generation.
	let temp_c:f64=rand::thread_rng().gen_range(1,50) as f64;
	let temp_k:f64=temp_c-AB_Z;
	
	//generate answer.
	let mut osmoles=0.0;
	for i in 0..comp_vec.len(){
		let mut solutes=0;
		for x in comp_vec[i].solutes.iter(){
			solutes+=x.0
		};
		osmoles+=m_comps[i]/comp_vec[i].mmass*(solutes as f64)
	};
	
	//generate osm_ity_a
	let mut osm_ity_a=0.0;
	for i in 1..comp_vec.len(){
		let mut solutes=0;
		for x in comp_vec[i].solutes.iter(){solutes+=x.0};
		osm_ity_a+=m_comps[i]/comp_vec[i].mmass*(solutes as f64)
	};
	osm_ity_a=osm_ity_a/v_litre;

	let osm_p=R*temp_k*osmoles/v_litre;
	let osm_ity:f64=osm_p/R/temp_k;
	let osm_ity_b:f64=osm_ity-osm_ity_a;
	let mut sol_x=0;
	for x in comp_vec[0].solutes.iter(){sol_x+=x.0};
	let sol_x:f64=sol_x as f64;

	//Generate question text.
	let question_a=format!("A solution with a volume of {} litres contains:",v_litre);
	let mut question_b:Vec<String>=Vec::new();
	for i in 1..no_comps{
		question_b.push(format!("{}g of {}...",
			dis(m_comps[i]),
			comp_vec[i].name[0]
		))
			
	};
	let question_b=question_b.join("\n");
	let question_c=format!("...And some {}.",comp_vec[0].name[0]);
	let question_d=format!("If the osmotic pressure is equal to {} KPa at {} degrees Celsius, what is the mass of {} in this solution?",
		dis(osm_p),
		temp_c,
		comp_vec[0].name[0]
	);
	let question = format!("{}\n{}\n{}\n{}\n",question_a,question_b,question_c,question_d);
	
	//Generate answer text.
	let ans_a=format!("Temperature (Kelvin) = {}",temp_k);	
	let ans_b=format!("Osmolarity (osmol/L) (all solutes) = {}",osm_ity);
	let ans_c=format!("Osmolarity (osmol/L) (all solutes apart from {}) = {}",comp_vec[0].name[0],osm_ity_a);
	let ans_d=format!("Osmolarity of {} (osmol/L) = {}",comp_vec[0].name[0],osm_ity_b);
	let ans_e=format!("Molarity of {} (mol/L)= {}",comp_vec[0].name[0],osm_ity_b/sol_x);
	let ans_f=format!("{}",format!("Answer = {}g",dis(m_comps[0])));
	let answer = format!("{}\n{}\n{}\n{}\n{}\n\n {}\n",ans_a,ans_b,ans_c,ans_d,ans_e,ans_f);
	(question,answer)
}

//IONIC SRENGTH QUESTION TYPES.
//IONIC SRENGTH QUESTION TYPES.
//IONIC SRENGTH QUESTION TYPES.
//IONIC SRENGTH QUESTION TYPES.



pub fn q_3_0(compounds:&Vec<Compound>)->(String,String){
//Question of type I=1/2*sum(cq^2).
//println!("q_3_0");
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;

	//generate compound
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.use_weak==false) || (x.salt==true){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	
	//generate mass of compound in question.
	let mut conc:f64=(rand::thread_rng().gen_range(10,601) as f64)/500.0;
	//println!("A");
	//Check solubility
	let mut silly=true;
	while silly==true{
		if conc*c.mmass/10.0>c.solubility{conc=conc/10.0}else{silly=false}
	};
	//println!("B");
	//generate answer.
	let mut spq=0.0;
	for x in c.solutes.iter(){
		spq+= conc*(x.0 as f64)*((abs(x.2)*abs(x.2)) as f64)/2.0;
	};
	
	//println!("C spq: {}",spq);
	let question = format!("What is the ionic strength of a solution with {}M {}?",
	dis(conc),
	c.name[0]);
	//println!("D");
	let mut answer_a:Vec<String>=Vec::new();
	answer_a.push("I = Σ(c x q^2)".to_owned());
	
	for x in c.solutes.iter(){
		if x.2!=0{
			answer_a.push(format!("Ion: {} x {}. (q^2 = {})",x.0,x.1,(x.2)*(x.2)))
		}else{
			answer_a.push(format!("{} is not ionic",x.1))
		}
	};
	//println!("D2");
	let answer_b=format!("{}",format!("Answer = {}",dis_u(spq)));
	let answer = format!("{}\n\n {}\n",answer_a.join("\n"),answer_b);
	//println!("E");
	(question,answer)
}


pub fn q_3_1(compounds:&Vec<Compound>)->(String,String){
//Question of type c=2*I/(sum(soln*q^2)).
//Aka reverse ionic strength question.
//println!("q_3_1");
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	
	//generate compound
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut qs_sum=0;
		for y in x.solutes.iter(){qs_sum+=(y.2)*(y.2)};
		if (qs_sum>1)
		 & ((x.use_weak==false) || (x.salt==true)){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	
	//generate mass of compound in question.
	let mut conc:f64=(rand::thread_rng().gen_range(10,601) as f64)/500.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if conc*c.mmass/10.0>c.solubility{conc=conc/10.0}else{silly=false}
	};
	
	//generate answer.
	let mut spq=0.0;
	for x in c.solutes.iter(){
		spq+= conc*(x.0 as f64)*(x.2 as f64)*(x.2 as f64)/2.0;
	};
		
	let question = format!("What is the molarity of a {} solution with an ionic strength of {}?",c.name[0],dis_u(spq));
	
	let mut answer_a:Vec<String>=Vec::new();
	answer_a.push("I = Σ(c x q^2)".to_owned());
	
	for x in c.solutes.iter(){
		if x.2!=0{
			answer_a.push(format!("Ion: {} x {}. (q^2 = {})",x.0,x.1,(x.2)*(x.2)));
			answer_a.push(format!("Therefore ([{}] x q^2)/C = {}",x.1,(x.0 as i8)*(x.2)*(x.2)))
		}else{
			answer_a.push(format!("{} is not ionic",x.1))
		}
	};
	let answer_b=format!("{}",format!("Answer = {}mol/L",dis(conc)));
	let answer_a=answer_a.join("\n");
	let answer = format!("{}\n\n {}\n",answer_a,answer_b);
	(question,answer)
}


pub fn q_3_2(compounds:&Vec<Compound>)->(String,String){
//Question of type I=Σ(m*Mr/V*q^2).
//println!("q_3_2");
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;

	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.use_weak==false) || (x.salt==true){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	
	//generate mass of compound in question.
	let mut m:f64=(rand::thread_rng().gen_range(10,2001) as f64)/200.0;
	let v_litre:f64=(rand::thread_rng().gen_range(10,2001) as f64)/500.0;
	//println!("A");
	//Check solubility
	let mut silly=true;
	while silly==true{
		if m/v_litre/10.0>c.solubility{m=m/10.0}else{silly=false};
	};	
	//println!("B");
	//generate answer.
	let conc=m/c.mmass/v_litre;
	let mut spq=0.0;
	for x in c.solutes.iter(){
		spq+= conc*(x.0 as f64)*((abs(x.2)*abs(x.2)) as f64)/2.0;
	};
	
	//Print Question.
	let question = format!("A solution with a volume of {}L contains {}g of {}. What is its ionic strength?",
	dis(v_litre),
	dis(m),
	c.name[0]);
	//println!("D");
	//Print Answer.
	let answer_a=format!("{} concentration = {} mol/L",c.name[0],&ff(4,conc));
	let mut answer_b:Vec<String>=Vec::new();
	
	answer_b.push("I = Σ(c x q^2)".to_owned());
	for x in c.solutes.iter(){
		if x.2!=0{
			answer_b.push(format!("Ion: {} x {}. (q^2 = {})",x.0,x.1,(x.2)*(x.2)))
		}else{
			answer_b.push(format!("{} is not ionic",x.1))
		}
	};
	//println!("D2");
	let answer_c=format!("{}",format!("Answer = {}",dis_u(spq)));
	let answer_b=answer_b.join("\n");
	let answer = format!("{}\n{}\n\n {}\n",answer_a,answer_b,answer_c);
	//println!("E");
	(question,answer)
}


pub fn q_3_2b(compounds:&Vec<Compound>)->(String,String){
//Question of type I=Σ(m*Mr/V*q^2). Variant
//FIND V!
//println!("q_3_2b");	
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	
	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut qs_sum=0;
		for y in x.solutes.iter(){qs_sum+=abs((y.2)*(y.2))};
		if (qs_sum>0)
		 & ((x.use_weak==false) || (x.salt==true))
		 & ((x.solubility/x.mmass)>0.000001){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	
	//check "silly physics" loops.
	let mut silly=true;
	let mut m:f64=0.0;
	let mut i_s:f64=0.0;
	let mut moles:f64=0.0;
	let mut solutes:f64;
	let mut conc:f64=0.0;
	let mut v_litre:f64=0.0;
	let mut f:f64=1.0;
		
	while silly==true{
		//generate mass of compound in question.
		m=(rand::thread_rng().gen_range(10,2001) as f64)/200.0;
		i_s=(rand::thread_rng().gen_range(10,2001) as f64)/500.0/f;
//		println!("i_s generated: {}",i_s);
	
		//generate answer.
		moles=m/c.mmass;
		let mut solutes_a=0.0;
		for x in c.solutes.iter(){
			solutes_a+= (x.0 as f64)*((abs(x.2)*abs(x.2)) as f64);
//			println!("solutes_a={}",solutes_a)
		};
		solutes=solutes_a;
//		println!("solutes value: {}\n solutes_a value: {}",solutes,solutes_a);
		conc=i_s*2.0/solutes;
		v_litre=moles/conc;
		
		//check solubility (silly version). 
		if m/v_litre/10.0>c.solubility{
			f=f*10.0;
		}else{silly=false}
	};
	
	//Print Question.

	let question = format!("{}g of {} are used to prepare a solution with an ionic strength of {}. What is its volume?",dis(m),c.name[0],dis_u(i_s));
	
	//Print Answer.
	let mut factor:usize=0;
	let mut ans_a:Vec<String>=Vec::new();
	
	ans_a.push("I = Σ(c x q^2)".to_owned());
	for x in c.solutes.iter(){
		if x.2!=0{
			ans_a.push(format!("Ion: {} x {}. (q^2 = {})",x.0,x.1,(x.2)*(x.2)));
			ans_a.push(format!("Therefore ([{}] x q^2)/C = {}",x.1,(x.0 as i8)*(x.2)*(x.2)));
			factor+=((x.0 as i8)*(x.2)*(x.2)) as usize;
		}else{
			ans_a.push(format!("{} is not ionic",x.1))
		}
	};
	let ans_a=ans_a.join("\n");
	let ans_b=format!("\n2 x I/C = {}",factor);
	let ans_c=format!("Therefore {} concentration = {} mol/L",c.name[0],ff(6,conc));
	let ans_d=format!("Moles = {}",moles);
	let ans_e=format!("{}",format!("Answer = {}L",dis(v_litre)));
	let answer = format!("{}\n{}\n{}\n{}\n\n {}\n",ans_a,ans_b,ans_c,ans_d,ans_e);
	(question,answer)
}


pub fn q_3_2c(compounds:&Vec<Compound>)->(String,String){
//Question of type I=Σ(m*Mr/V*q^2). Variant
//FIND m!
//println!("q_3_2c");
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;

	let mut v_valid:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut qs_sum=0;
		for y in x.solutes.iter(){qs_sum+=(y.2)*(y.2)};
		if (qs_sum>0)
		 & ((x.use_weak==false) || (x.salt==true))
		 & ((x.solubility/x.mmass)>0.000001){v_valid.push(x)
		}else{}
	};
	let c_len=v_valid.len();
	
	//check "silly physics" loops. And initiate variables.
	let mut silly=true;	
	let mut m=0.0;
	let mut i_s=0.0;
	let mut moles;
	let mut solutes;
	let mut conc=0.0;
	let mut v_litre=0.0;
	let indx=rand::thread_rng().gen_range(0,c_len);
	let c=&v_valid[indx];
	let mut f=1.0;
	
	while silly==true{		
		//generate compound
		
	
		//generate mass of compound in question.
		v_litre=(rand::thread_rng().gen_range(10,2001) as f64)/500.0;
		i_s=(rand::thread_rng().gen_range(10,2001) as f64)/500.0/f;
//		println!("i_s generated: {}",i_s);
	
		//generate answer.
		let mut solutes_a:f64=0.0;
		for x in c.solutes.iter(){
			solutes_a+= (x.0 as f64)*((abs(x.2)*abs(x.2)) as f64);
//			println!("solutes_a={}",solutes_a)
		};
		solutes=solutes_a;
//		println!("solutes value: {}\n solutes_a value: {}",solutes,solutes_a);
		conc=i_s*2.0/solutes;
		moles=conc*v_litre;
		m=moles*c.mmass;
	
		//check solubility (silly version). 
		if m/v_litre/10.0>c.solubility{f=f*10.0}else{silly=false}
	};
	
	//Print Question.
	let question = format!("A {} solution was found to have an ionic strength of {}. How many grams of {} do {}L of solution contain?",
		c.name[0],
		dis_u(i_s),
		c.name[0],
		dis(v_litre));
	
	//Print Answer.
	let mut ans_a:Vec<String>=Vec::new();
	let mut factor:usize=0;
	
	ans_a.push("I = Σ(c x q^2)".to_owned());
	for x in c.solutes.iter(){
		if x.2!=0{
			ans_a.push(format!("Ion: {} x {}. (q^2 = {})",x.0,x.1,(x.2)*(x.2)));
			ans_a.push(format!("Therefore ([{}] x q^2)/C = {}",x.1,(x.0 as i8)*(x.2)*(x.2)));
			factor+=((x.0 as i8)*(x.2)*(x.2)) as usize;
		}else{
			ans_a.push(format!("{} is not ionic",x.1))
		}
	};
	ans_a.push(format!("\n2 x I/C= {}",factor));
	ans_a.push(format!("Therefore {} concentration= {} mol/L",c.name[0],&ff(4,conc)));
	let ans_b=format!("{}",format!("Answer = {}g",dis(m)));
	let ans_a=ans_a.join("\n");
	let answer = format!("{}\n\n {}\n",ans_a,ans_b);
	(question,answer)
}



pub fn q_4_0(compounds:&Vec<Compound>)->(String,String){
//Calculate Ksp from solubility
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;

	
	//generate compound. NB for now compounds MUST have more than one solute.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solubility!=INFINITY)
		 & (x.salt==true)
		 & (x.solutes.len()==2)
		 & ((x.solubility/x.mmass)<0.2)
		 & ((x.solubility/x.mmass)>0.0000001) {valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	let variable:f64=1.0+(rand::thread_rng().gen_range(-50,51) as f64)/100.0;
	
	//generate solubility
	let s_m_100:f64=match ff(4,c.solubility*variable).trim().parse(){
		Ok(num)=>num,
		Err(_)=>c.solubility*variable,
	};	
	
	//generate answer.
	let s_c:f64=s_m_100*10.0/c.mmass;
	let mut solutes_vec:Vec<(usize,f64,&str)>=Vec::new();
	for x in c.solutes.iter(){
		solutes_vec.push((x.0 as usize,x.0 as f64,&x.1))  //NB: This assumes that zero charge (non ionic) solutes contribute to Ksp.
	};
	let mut s_c_vec:Vec<(f64)>=Vec::new();
	for x in solutes_vec.iter(){
		s_c_vec.push((x.1*s_c).powf(x.1))
	};
	let answer=s_c_vec.iter().fold(1.0,|product,&x|product*x);
	let multiple=solutes_vec.iter().fold(1,|product,&x|product*(x.0).pow(x.0 as u32));
	let power=solutes_vec.iter().fold(0,|sum,&x|sum+x.0);
	let power:String= if power==1{"".to_owned()}else{format!("{}",power)};
	let multiple:String= if multiple==1{"".to_owned()}else{format!("{}",multiple)};
	
	//PRINT QUESTION
	let question = format!("What is the Ksp of {} if its solubility under a given set of conditions is {}g/100mL?",
		c.name[0],
		dis(s_m_100));
	
	//PRINT ANSWER
	let mut ans_a:Vec<String>=Vec::new();
	ans_a.push(format!("s = {} mol/L\n",s_c));
	ans_a.push(format!("Ksp = "));
	for i in 0..(s_c_vec.len()){
		if i==0{
			ans_a.push(format!("[{}]^{}",solutes_vec[i].2,solutes_vec[i].0))
		}else{
			ans_a.push(format!(" x [{}]^{}",solutes_vec[i].2,solutes_vec[i].0))
		};
	};
	ans_a.push(format!("\nKsp = "));
	for i in 0..(s_c_vec.len()){
		if i==0{
			ans_a.push(format!("{}s^{}",(solutes_vec[i].0).pow(solutes_vec[i].0 as u32),solutes_vec[i].0))
		}else{
			ans_a.push(format!(" x {}s^{}",(solutes_vec[i].0).pow(solutes_vec[i].0 as u32),solutes_vec[i].0))
		};
	};
	ans_a.push(format!("\nKsp = {}s^{}",multiple,power));
	let mut ans_b:Vec<String>=Vec::new();
	ans_b.push(format!("{}",format!("Answer = {}",dis_u(answer))));
	if c.solubility*10.0/c.mmass>1.0{
		ans_b.push(format!("NB, this method for calculating Ksp should not be used for highly soluble compounds like {}.",
		c.name[0]))
	}else{
		ans_b.push(format!(""))
	};
	let ans_a=ans_a.join("");
	let ans_b=ans_b.join("\n");
	let answer = format!("{}\n\n {}\n",ans_a,ans_b);
	(question,answer)	
}


pub fn q_4_0a(compounds:&Vec<Compound>)->(String,String){
//Calculate solubility from Ksp.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;

	
	//generate compound. NB for now compounds MUST have more than one solute.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solubility!=INFINITY)
		 & (x.salt==true)
		 & (x.solutes.len()==2)
		 & ((x.solubility/x.mmass)<0.2)
		 & ((x.solubility/x.mmass)>0.0000001)  {valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	let variable:f64=1.0+(rand::thread_rng().gen_range(-50,51) as f64)/100.0;
	
	//generate solubility (NB this reparsing step makes the answer more "exact".)-not perfect for this kind of question.
	let s_m_100:f64=match ff(4,c.solubility*variable).trim().parse(){
		Ok(num)=>num,
		Err(_)=>c.solubility*variable,
	};	
	
	//generate answer.
	let s_c:f64=s_m_100*10.0/c.mmass;
	let mut solutes_vec:Vec<(usize,f64,&str)>=Vec::new();
	for x in c.solutes.iter(){
		solutes_vec.push((x.0 as usize,x.0 as f64,&x.1))  //NB: This assumes that zero charge (non ionic) solutes contribute to Ksp.
	};
	let mut s_c_vec:Vec<(f64)>=Vec::new();
	for x in solutes_vec.iter(){
		s_c_vec.push((x.1*s_c).powf(x.1))
	};
	let answer=s_c_vec.iter().fold(1.0,|product,&x|product*x);
	let multiple=solutes_vec.iter().fold(1,|product,&x|product*(x.0).pow(x.0 as u32));
	let power=solutes_vec.iter().fold(0,|sum,&x|sum+x.0);
	let power:String= if power==1{"".to_owned()}else{format!("{}",power)};
	let multiple:String= if multiple==1{"".to_owned()}else{format!("{}",multiple)};
	
	//PRINT QUESTION
	let question = format!("What is the solubility (in g/100mL) of {} if its Ksp is {}?",
	c.name[0],dis_u(answer));
	
	//PRINT ANSWER
	let mut ans_a:Vec<String>=Vec::new();
	ans_a.push(format!("Ksp = "));
	for i in 0..(s_c_vec.len()){
		if i==0{
			ans_a.push(format!("[{}]^{}",solutes_vec[i].2,solutes_vec[i].0))
		}else{
			ans_a.push(format!(" x [{}]^{}",solutes_vec[i].2,solutes_vec[i].0));
		};
	};
	ans_a.push(format!("\nKsp = "));
	for i in 0..(s_c_vec.len()){
		if i==0{
			ans_a.push(format!("{}s^{}",(solutes_vec[i].0).pow(solutes_vec[i].0 as u32),solutes_vec[i].0));
		}else{
			ans_a.push(format!(" x {}s^{}",(solutes_vec[i].0).pow(solutes_vec[i].0 as u32),solutes_vec[i].0));
		};
	};
	ans_a.push(format!("\nKsp = {}s^{}\n",multiple,power));
	if multiple==""{
		ans_a.push(format!("s = {}(Ksp{})\n",form_root(power),multiple));
	}else{
		ans_a.push(format!("s = {}(Ksp/{})\n",form_root(power),multiple));
	};
	
	ans_a.push(format!("s = {}mol/L",dis(s_c)));
	let ans_a=ans_a.join("");
	
	let mut ans_b:Vec<String>=Vec::new();
	ans_b.push(format!("{}",format!("Answer = {}g/100mL",dis(s_m_100))));
	if c.solubility*10.0/c.mmass>1.0{
		ans_b.push(format!("NB, this method for calculating Ksp should not be used for highly soluble compounds like {}.",c.name[0]));
	}else{
		ans_b.push(format!(""));
	};
	let ans_b=ans_b.join("\n");
	let answer = format!("{}\n\n {}\n",ans_a,ans_b);
	(question,answer)	
}

//NB This question is not in the general form. only works for binary ions.

pub fn q_4_1(compounds:&Vec<Compound>)->(String,String){
//Calculate concentration of one ion from Ksp and concentration of the other.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;

	
	//generate compound. NB for now compounds MUST have more than one solute.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solubility!=INFINITY)
		 & (x.salt==true)
		 & (x.solutes.len()==2)
		 & ((x.solubility/x.mmass)<0.05)
		 & ((x.solubility/x.mmass)>0.0000000001)  {valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	let variable:f64=1.0+(rand::thread_rng().gen_range(-50,51) as f64)/100.0;
	let var_b:f64=1.0+(rand::thread_rng().gen_range(1,100) as f64)/10.0;
	
	//generate solubility
	let s_m_100:f64=match ff(4,c.solubility*variable).trim().parse(){
		Ok(num)=>num,
		Err(_)=>c.solubility*variable,
	};	
	let s_c:f64=s_m_100*10.0/c.mmass;
	
	//generate ion to solve for.
	let ss_len=c.solutes.len();
	let wch_ion:usize=rand::thread_rng().gen_range(0,ss_len);
	let mut known:(&str,f64,u8)=("",0.0,0);
	let mut unknown:(&str,f64,u8)=("",0.0,0);
	for i in 0..ss_len{
		if i==wch_ion{
			known=(&c.solutes[i].1,c.solutes[i].0 as f64,c.solutes[i].0)
		}else{
			unknown=(&c.solutes[i].1,c.solutes[i].0 as f64,c.solutes[i].0)
		}
	};
	
	//generate concentration of known ion;
	let c_known:f64= match ff(4,s_c*known.1*var_b).trim().parse(){
		Ok(num)=>num,
		Err(_)=>s_c*known.1*var_b,
	};
	
	//generate Ksp.
	let mut solutes_vec:Vec<(usize,f64,&str)>=Vec::new();
	for x in c.solutes.iter(){
		solutes_vec.push((x.0 as usize,x.0 as f64,&x.1))  //NB: This assumes that zero charge (non ionic) solutes contribute to Ksp.
	};
	let mut s_c_vec:Vec<(f64)>=Vec::new();
	for x in solutes_vec.iter(){
		s_c_vec.push((x.1*s_c).powf(x.1))
	};
	let ksp=s_c_vec.iter().fold(1.0,|product,&x|product*x);
	//generate concentration of known ion;
	let ksp:f64= match ff(4,ksp).trim().parse(){
		Ok(num)=>num,
		Err(_)=>ksp,
	};
	//let multiple=solutes_vec.iter().fold(1,|product,&x|product*(x.0).pow(x.0 as u32));
	//let power=solutes_vec.iter().fold(0,|sum,&x|sum+x.0);
	//let power:String= if power==1{"".to_owned()}else{format!("{}",power)};
	//let multiple:String= if multiple==1{"".to_owned()}else{format!("{}",multiple)};
	
	let c_unknown=(ksp/(c_known).powf(known.1)).powf(1.0/unknown.1);
	
	//PRINT QUESTION
	let question = format!("\
			Under a given set of conditions, {} has a Ksp of {}. \
			If [{}] is {}mol/L, at what molar concentration of {} \
			will it begin to precipitate out of the solution?",
			c.name[0],dis_u(ksp),known.0,dis(c_known),unknown.0);
	
	//PRINT ANSWER
	let mut ans_a:Vec<String>=Vec::new();
	ans_a.push(format!("Ksp = "));
	for i in 0..(s_c_vec.len()){
		if i==0{
			ans_a.push(format!("[{}]^{}",solutes_vec[i].2,solutes_vec[i].0));
		}else{
			ans_a.push(format!(" x [{}]^{}",solutes_vec[i].2,solutes_vec[i].0));
		};
	};
	ans_a.push(format!("\n[{}]^{} = Ksp/[{}]^{}",unknown.0,unknown.2,known.0,known.2));
	if unknown.2>1 {
		ans_a.push(format!("\n[{}] = {}(Ksp/[{}]^{})",unknown.0,form_root(unknown.2.to_string()),known.0,known.2));
	};
	//println!("\nKsp = {}s^{}",multiple,power);
	//println!("s=(Ksp/{})^(1/{})",multiple,power);

	let ans_b=format!("{}",format!("Answer = {}mol/L\n",dis(c_unknown)));
	let ans_a=ans_a.join("");
	let answer = format!("{}\n\n {}\n",ans_a,ans_b);
	(question,answer)		
}

//NB This question is not in the general form. only works for binary ions.

pub fn q_4_1b(compounds:&Vec<Compound>)->(String,String){
//Calculate concentration of one ion from solubility and concentration of the other.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;

	
	//generate compound. NB for now compounds MUST have more than one solute.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solubility!=INFINITY)
		 & (x.salt==true)
		 & (x.solutes.len()==2)
		 & ((x.solubility/x.mmass)<0.05)
		 & ((x.solubility/x.mmass)>0.0000000001)  {valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	let variable:f64=1.0+(rand::thread_rng().gen_range(-50,51) as f64)/100.0;
	let var_b:f64=1.0+(rand::thread_rng().gen_range(1,100) as f64)/10.0;
	
	//generate solubility (this should make answer more accurate).
	let s_m_100:f64=match ff(4,c.solubility*variable).trim().parse(){
		Ok(num)=>num,
		Err(_)=>c.solubility*variable,
	};	
	let s_c:f64=s_m_100*10.0/c.mmass;
	
	//generate ion to solve for.
	let ss_len=c.solutes.len();
	let wch_ion:usize=rand::thread_rng().gen_range(0,ss_len);
	let mut known:(&str,f64,u8)=("",0.0,0);
	let mut unknown:(&str,f64,u8)=("",0.0,0);
	for i in 0..ss_len{
		if i==wch_ion{
			known=(&c.solutes[i].1,c.solutes[i].0 as f64,c.solutes[i].0)
		}else{
			unknown=(&c.solutes[i].1,c.solutes[i].0 as f64,c.solutes[i].0)
		}
	};
	
	//generate concentration of known ion (this should make answer more accurate).;
	let c_known:f64= match ff(4,s_c*known.1*var_b).trim().parse(){
		Ok(num)=>num,
		Err(_)=>s_c*known.1*var_b,
	};
	
	//generate Ksp.
	let mut solutes_vec:Vec<(usize,f64,&str)>=Vec::new();
	for x in c.solutes.iter(){
		solutes_vec.push((x.0 as usize,x.0 as f64,&x.1))  //NB: This assumes that zero charge (non ionic) solutes contribute to Ksp.
	};
	let mut s_c_vec:Vec<(f64)>=Vec::new();
	for x in solutes_vec.iter(){
		s_c_vec.push((x.1*s_c).powf(x.1))
	};
	let ksp=s_c_vec.iter().fold(1.0,|product,&x|product*x);
	
	let multiple=solutes_vec.iter().fold(1,|product,&x|product*(x.0).pow(x.0 as u32));
	let power=solutes_vec.iter().fold(0,|sum,&x|sum+x.0);
	let power:String= if power==1{"".to_owned()}else{format!("{}",power)};
	let multiple:String= if multiple==1{"".to_owned()}else{format!("{}",multiple)};
	
	let c_unknown=(ksp/(c_known).powf(known.1)).powf(1.0/unknown.1);
	
	//PRINT QUESTION
	let question = format!("\
			The solubility of {} under a given set of conditions is {}g/100mL. \
			If [{}] is {}mol/L, at what [{}] will {} \
			begin to precipitate out of the solution?",
			c.name[0],dis(s_m_100),known.0,dis(c_known),unknown.0,unknown.0);
	
	//PRINT ANSWER
	let mut ans_a:Vec<String>=Vec::new();
	ans_a.push(format!("Ksp = "));
	for i in 0..(s_c_vec.len()){
		if i==0{
			ans_a.push(format!("[{}]^{}",solutes_vec[i].2,solutes_vec[i].0));
		}else{
			ans_a.push(format!(" x [{}]^{}",solutes_vec[i].2,solutes_vec[i].0));
		};
	};
	ans_a.push(format!("\nKsp = {}s^{}",multiple,power));
	ans_a.push(format!("\nKsp = {}",dis_u(ksp)));
	ans_a.push(format!("\n[{}]^{} = Ksp/[{}]^{}",unknown.0,unknown.2,known.0,known.2));
	if unknown.2>1 {
		ans_a.push(format!("\n[{}] = {}(Ksp/[{}]^{})",unknown.0,form_root(unknown.2.to_string()),known.0,known.2));
	};
	
	//println!("\nKsp = {}s^{}",multiple,power);
	//println!("s=(Ksp/{})^(1/{})",multiple,power);

	let ans_b=format!("{}",format!("Answer = {}mol/L",dis(c_unknown)));
	let ans_a=ans_a.join("");
	let answer = format!("{}\n\n {}\n",ans_a,ans_b);
	(question,answer)	
}

//pH strong
//pH strong (THIS FUNCTION IS OK)

pub fn q_6_0(compounds:&Vec<Compound>)->(String,String){
//Find pH from concentration.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;

	
	//Generate strong acid or base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solutes.len()==2)
		 & ((x.pka[0].0>8.0)||(x.pka[0].0<6.0)){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	//generate concentration.
	let mut conc:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if c.solubility==f64::INFINITY{silly=false; continue
		}else{
			if conc*c.mmass/10.0>c.solubility{conc=conc/25.0}else{silly=false}
		}
	};
	
	//Extra decimal space removal post solubility check.
	let conc:f64= match ff(4,conc).trim().parse(){
		Ok(num)=>num,
		Err(_)	 =>conc,
	};
	
	let mut eff_conc=0.0;
	let mut acid=true;
	
	//generate answer. (Determine if acid or base and whether it is a salt to boot. Determine effective concentration.
	let p_h;
	for x in c.solutes.iter(){
		if (x.1==c.pka[0].1) & (c.pka[0].0<7.0){
			let mut weak_acid_salt=true;
			for y in c.solutes.iter(){
				if y.1=="H"{weak_acid_salt=false}else{}
			};
			acid=if (c.use_weak==false) || (weak_acid_salt==false){true}else{false};
			eff_conc=conc*(x.0 as f64)*(abs(x.2) as f64);
		}else if (x.1==c.pka[0].1) & (c.pka[0].0>7.0){
			let mut weak_base_salt=true;
			for y in c.solutes.iter(){
				if y.1=="OH"{weak_base_salt=false}else{}
			};
			acid= if (c.use_weak==false) || (weak_base_salt==false){false}else{true};
			eff_conc=conc*(x.0 as f64)*(abs(x.2) as f64);
		}else{}
	};
	
	//generate answer. (Use strong/weak acid/base formula to determine pH)
	if acid==true{
		p_h= if c.use_weak==false {0.0-(eff_conc).log(10.0)}
			else {0.5*(c.pka[0].0-(eff_conc).log(10.0))}
	}else{
		p_h= if c.use_weak==false {14.0+(eff_conc).log(10.0)}
			else {7.0+0.5*(c.pka[0].0+(eff_conc).log(10.0))}
	};
	
	//Print Question.
	let question = format!("If the concentration of {} is {}mol/L, what is its pH?",
	c.name[0],dis(conc));
	
	//Print Answer.
	let mut ans_a=Vec::new();
	if c.use_weak==false{
		ans_a.push(format!("pH = -log[H+]"));
		if acid==false{ans_a.push(format!("pOH = -log[OH-]\npH = 14-pOH"))}
	}else{
		if acid==true{
			ans_a.push(format!("This compound acts like a weak acid."))
		}else{
			ans_a.push(format!("This compound acts like a weak base."))
		}
	};
	let ans_b=format!("{}",format!("pH = {}",&ff(4,p_h)));
	let answer = format!("{}\n\n {}\n",ans_a.join("\n"),ans_b);
	(question,answer)
}

//THIS FUNCTION IS NOW FINE (But can give above limit concentrations)

pub fn q_6_0b(compounds:&Vec<Compound>)->(String,String){
//Find concentration. from pH
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;

	
	//Generate acid or base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solutes.len()==2)
		 & ((x.pka[0].0>8.0)||(x.pka[0].0<6.0)){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	let mut acid:bool=true;
	let mut n:usize=0;
	
	//Decide which method to use (Determine if acid or base and whether it is a salt to boot. Determine effective concentration.
	for x in c.solutes.iter(){
		if (x.1==c.pka[0].1) & (c.pka[0].0<7.0){
			let mut weak_acid_salt=true;
			for y in c.solutes.iter(){
				if y.1=="H"{weak_acid_salt=false}else{}
			};
			acid=if (c.use_weak==false) || (weak_acid_salt==false){true}else{false};
			n=((x.0 as f64)*(abs(x.2) as f64)) as usize
		}else if (x.1==c.pka[0].1) & (c.pka[0].0>7.0){
			let mut weak_base_salt=true;
			for y in c.solutes.iter(){
				if y.1=="OH"{weak_base_salt=false}else{}
			};
			acid= if (c.use_weak==false) || (weak_base_salt==false){false}else{true};
			n=((x.0 as f64)*(abs(x.2) as f64)) as usize
		}else{}
	};
	
	let n_f64=n as f64;
	let pre_p_h=((rand::thread_rng().gen_range(0,4001)-800) as f64)/1000.0;
	
	//generate answer. (Use strong/weak acid/base formula to determine pH)
	let p_h;
	if acid==true{
		p_h= if c.use_weak==false {pre_p_h}
			else {0.5*(c.pka[0].0+pre_p_h)}
	}else{
		p_h= if c.use_weak==false {14.0-pre_p_h}
			else {7.0+0.5*(c.pka[0].0-pre_p_h)}
	};
	
	let p_h= match ff(4,p_h).trim().parse(){
		Ok(num)=>num,
		Err(_)=>p_h,
	};
	
	let conc;
	if acid==true{
		conc= if c.use_weak==false {TEN.powf(0.0-p_h)/n_f64}
			else {TEN.powf(c.pka[0].0-2.0*p_h)/n_f64}
	}else{
		conc= if c.use_weak==false {TEN.powf(p_h-14.0)/n_f64}
			else {TEN.powf((p_h-7.0)*2.0-c.pka[0].0)/n_f64}
	};
	
	//Print Question.
		let question = format!("A solution of {} has a pH of {}, what is its concentration?",
		c.name[0],&ff(4,p_h));
	
	//Print Answer.
	let mut ans_a=Vec::new();
	if (acid==true) & (c.use_weak==false){
		ans_a.push(format!("pH = -log[H+]"));
		ans_a.push(format!("[H+] = 10^(-pH)"));
		ans_a.push(format!("{} x c = 10^(-pH)",n));
	}else if c.use_weak==false{
		ans_a.push(format!("pOH = -log[OH-]\n-log[OH-] = 14-pH"));
		ans_a.push(format!("[OH-] = 10^(pH-14)"));
		ans_a.push(format!("{} x c = 10^(pH-14)",n));
	}else if (acid==true) & (c.use_weak==true){
		ans_a.push(format!("pH = 0.5 x (pKa - log({} x c))",n));
		ans_a.push(format!("log({} x c) = pKa - (2 x pH)",n));
	}else{
		ans_a.push(format!("pH = 7 + 0.5 x (pKa + log({} x c))",n));
		ans_a.push(format!("log({} x c) = 2 x (pH - 7) - pKa",n));
	};
	let ans_b=format!("{}",format!("Answer = {}mol/L",dis(conc)));
	let ans_c=if conc>c.solubility*c.mmass/10.0 {
		format!("(This is a slightly silly question because the \"correct\" answer exceeds the compound's solubility...)")
	}else{
		format!("")
	};
	let answer = format!("{}\n\n {}\n{}\n",ans_a.join("\n"),ans_b,ans_c);
	(question,answer)
}

//pH strong (THIS FUNCTION IS OK)

pub fn q_6_1(compounds:&Vec<Compound>)->(String,String){
//Find pH from mass.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	
	
	//Generate strong acid or base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solutes.len()==2)
		 & ((x.pka[0].0>8.0)||(x.pka[0].0<6.0)){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	//generate concentration.
	let mut conc:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	//generate volume.
	let v_litre:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if c.solubility==f64::INFINITY{silly=false; continue
		}else{
			if conc*c.mmass/10.0>c.solubility{conc=conc/10.0}else{silly=false}
		}
	};
	
	//Extra decimal space removal post solubility check.
	let conc:f64= match ff(4,conc).trim().parse(){
		Ok(num)=>num,
		Err(_)	 =>conc,
	};
	
	
	//find mass.
	let m=conc*c.mmass*v_litre;
	
	let mut eff_conc=0.0;
	let mut acid=true;
	
	//generate answer. (Determine if acid or base and whether it is a salt to boot. Determine effective concentration.
	let p_h;
	for x in c.solutes.iter(){
		if (x.1==c.pka[0].1) & (c.pka[0].0<7.0){
			let mut weak_acid_salt=true;
			for y in c.solutes.iter(){
				if y.1=="H"{weak_acid_salt=false}else{}
			};
			acid=if (c.use_weak==false) || (weak_acid_salt==false){true}else{false};
			eff_conc=conc*(x.0 as f64)*(abs(x.2) as f64);
		}else if (x.1==c.pka[0].1) & (c.pka[0].0>7.0){
			let mut weak_base_salt=true;
			for y in c.solutes.iter(){
				if y.1=="OH"{weak_base_salt=false}else{}
			};
			acid= if (c.use_weak==false) || (weak_base_salt==false){false}else{true};
			eff_conc=conc*(x.0 as f64)*(abs(x.2) as f64);
		}else{}
	};
	
	//generate answer. (Use strong/weak acid/base formula to determine pH)
	if acid==true{
		p_h= if c.use_weak==false {0.0-(eff_conc).log(10.0)}
			else {0.5*(c.pka[0].0-(eff_conc).log(10.0))}
	}else{
		p_h= if c.use_weak==false {14.0+(eff_conc).log(10.0)}
			else {7.0+0.5*(c.pka[0].0+(eff_conc).log(10.0))}
	};
	
	//Print Question.
	let question = format!("A solution contains {}g of {} in {}L of solution, what is its pH?",
	dis(m),c.name[0],dis(v_litre));
	
	//Print Answer.
	let mut ans_a=Vec::new();
	ans_a.push(format!("Concentration of {}: {} mol/L",c.name[0],conc));
	if c.use_weak==false{
		ans_a.push(format!("pH = -log[H+]"));
		if acid==false{ans_a.push(format!("pOH = -log[OH-]\npH = 14-pOH"));}
	}else{
		if acid==true{
			ans_a.push(format!("This compound acts like a weak acid."));
		}else{
			ans_a.push(format!("This compound acts like a weak base."));
		}
	};
	let ans_b=format!("{}",format!("pH = {}",&ff(4,p_h)));
	let answer = format!("{}\n\n {}\n",ans_a.join("\n"),ans_b);
	(question,answer)
}

//THIS FUNCTION should now give OK answers.

pub fn q_6_1b(compounds:&Vec<Compound>)->(String,String){
//Find mass. from pH

	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	
	//Generate acid or base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		if (x.solutes.len()==2)
		 & ((x.pka[0].0>8.0)||(x.pka[0].0<6.0)){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	let v_litre:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	let mut acid:bool=true;
	let mut n:usize=0;
	
	//Decide which method to use (Determine if acid or base and whether it is a salt to boot. Determine effective concentration.
	for x in c.solutes.iter(){
		if (x.1==c.pka[0].1) & (c.pka[0].0<7.0){
			let mut weak_acid_salt=true;
			for y in c.solutes.iter(){
				if y.1=="H"{weak_acid_salt=false}else{}
			};
			acid=if (c.use_weak==false) || (weak_acid_salt==false){true}else{false};
			n=((x.0 as f64)*(abs(x.2) as f64)) as usize
		}else if (x.1==c.pka[0].1) & (c.pka[0].0>7.0){
			let mut weak_base_salt=true;
			for y in c.solutes.iter(){
				if y.1=="OH"{weak_base_salt=false}else{}
			};
			acid= if (c.use_weak==false) || (weak_base_salt==false){false}else{true};
			n=((x.0 as f64)*(abs(x.2) as f64)) as usize
		}else{}
	};
	
	let n_f64=n as f64;
	let pre_p_h=((rand::thread_rng().gen_range(0,4001)-800) as f64)/1000.0;
	
	//generate answer. (Use strong/weak acid/base formula to determine pH)
	let p_h;
	if acid==true{
		p_h= if c.use_weak==false {pre_p_h}
			else {0.5*(c.pka[0].0+pre_p_h)}
	}else{
		p_h= if c.use_weak==false {14.0-pre_p_h}
			else {7.0+0.5*(c.pka[0].0-pre_p_h)}
	};
	
	let p_h= match ff(4,p_h).trim().parse(){
		Ok(num)=>num,
		Err(_)=>p_h,
	};
	
	let conc;
	if acid==true{
		conc= if c.use_weak==false {TEN.powf(0.0-p_h)/n_f64}
			else {TEN.powf(c.pka[0].0-2.0*p_h)/n_f64}
	}else{
		conc= if c.use_weak==false {TEN.powf(p_h-14.0)/n_f64}
			else {TEN.powf((p_h-7.0)*2.0-c.pka[0].0)/n_f64}
	};
	
	let m:f64=conc*v_litre*c.mmass;
	
	//Print Question.
	let question = format!("The pH of a {} solution is {}. What is the mass of {} in {}L of the solution?",
	c.name[0],&ff(4,p_h),c.name[0],dis(v_litre));
	
	//Print Answer.
	let mut ans_a=Vec::new();
	ans_a.push(format!("Concentration of {}: {} mol/L",c.name[0],conc));
	if (acid==true) & (c.use_weak==false){
		ans_a.push(format!("pH = -log[H+]"));
		ans_a.push(format!("[H+] = 10^(-pH)"));
		ans_a.push(format!("{} x c = 10^(-pH)",n));
	}else if c.use_weak==false{
		ans_a.push(format!("pOH = -log[OH-]\n-log[OH-] = 14-pH"));
		ans_a.push(format!("[OH-] = 10^(pH-14)"));
		ans_a.push(format!("{} x c = 10^(pH-14)",n));
	}else if (acid==true) & (c.use_weak==true){
		ans_a.push(format!("pH = 0.5 x (pKa - log({} x c))",n));
		ans_a.push(format!("log({} x c) = pKa - (2 x pH)",n));
	}else{
		ans_a.push(format!("pH = 7 + 0.5 x (pKa + log({} x c))",n));
		ans_a.push(format!("log({} x c) = 2 x (pH - 7) - pKa",n));
	};
	let ans_c=if conc>c.solubility*c.mmass/10.0 {
		format!("(This is a slightly silly question because the \"correct\" answer exceeds the compound's solubility...)")
	}else{
		format!("")
	};
	let ans_b=format!("{}",format!("Answer = {}g",dis(m)));
	let answer = format!("{}\n\n {}\n{}\n",ans_a.join("\n"),ans_b,ans_c);
	(question,answer)
}


//THIS FUNCTION SHOULD BE OK.

pub fn q_6_2a(compounds:&Vec<Compound>)->(String,String){
//Reaction between strong acids and bases.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;



	//set bronsted acid & base. (name,x/acid,x/c)
	let mut a_bron:(&str,u8,u8)=("H",1,1);
	let mut b_bron:(&str,u8,u8)=("OH",1,1);
	
	//Generate acid.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		let mut acid=false;
		//let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		if ((gives_h==true) & (x.pka[0].0<6.0))
		|| ((gives_h==false) & (x.salt==true) & (x.use_weak==true) & (x.pka[0].0>8.0)){acid=true}else{};
		if (x.solutes.len()==2)
		 & (acid==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	let strong_acid=if a.use_weak==true {false}else{true};
	
	
	//Generate base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut base=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if (y.1=="HCO3")||(y.1=="OH")||(y.1=="CO3"){gives_oh=true}else{}
		};
		if ((gives_oh==true) & (x.pka[0].0>8.0))
		|| ((gives_oh==false) & (x.salt==true) & (x.use_weak==true)& (x.pka[0].0<6.0)){base=true}else{};
		if (base==true)
		 & ((x.use_weak==false)||(strong_acid==true)){
			valid_c.push(&x)
		}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let b=&valid_c[indx];
	
	//determine bronsted acid (name,acid/acid,x/c)
	if a.salt==false{
		for x in a.solutes.iter(){
				if x.1=="H" {a_bron=("H",1,x.0)}else{}
		};
	}else{
		for x in a.solutes.iter(){
				if x.1==a.pka[0].1 {a_bron=("H",1,x.0)}else{}
		};
	};
	
	//determine bronsted base (name,acid/base,x/c)
	for x in b.solutes.iter(){
		let aob=(abs(x.2)) as u8;
		if (x.1=="OH")||(x.1=="HCO3")||(x.1=="CO3")||((x.1==b.pka[0].1) & b.salt & b.use_weak){b_bron=(&x.1,aob,x.0)}else{}
	};
	
	//generate concentration.
	let mut c_a:f64=(rand::thread_rng().gen_range(25,251) as f64)/500.0;
	let mut c_b:f64=(rand::thread_rng().gen_range(25,251) as f64)/500.0;
	
	//Some bases are almost insoluble so this reduces acid concentration to match:
	let mut silly=true;
	while silly==true{
		if b.solubility==f64::INFINITY{silly=false; continue
		}else{
			if c_a/10.0>b.solubility/b.mmass{c_a=c_a/10.0}else{silly=false}
		}
	};
	//Extra decimal space removal post solubility check.
	let c_a:f64= match ff(4,c_a).trim().parse(){
		Ok(num)=>num,
		Err(_)	 =>c_a,
	};
	
	//Check solubility of base.
	let mut silly=true;
	while silly==true{
		if b.solubility==f64::INFINITY{silly=false; continue
		}else{
			if c_b*b.mmass/10.0>b.solubility{c_b=c_b/10.0}else{silly=false}
		}
	};
	
	//Extra decimal space removal post solubility check.
	let c_b:f64= match ff(4,c_b).trim().parse(){
		Ok(num)=>num,
		Err(_)	 =>c_b,
	};
	
	//generate volume of acid (v_a) and base (v_b).
	let v_a=(rand::thread_rng().gen_range(30,1201) as f64)/1000.0;
	let v_b=(rand::thread_rng().gen_range(30,1201) as f64)/1000.0;
	
	//calculate moles H+ and B-
	let mol_h=c_a*(a_bron.2 as f64)*v_a;
	let mol_oh=c_b*((b_bron.2*b_bron.1) as f64)*v_b;
	
	//calculate moles remaining.
	let molf=absf64(mol_h-mol_oh);
	
	//get final pH
	let p_h;
	if (a.use_weak==false) & (b.use_weak==false){  //strong acid, strong base.
		p_h= if mol_h>mol_oh{-(molf/(v_a+v_b)).log(10.0)				//excess of acid. Strong acid.
			}else if mol_h<mol_oh {14.0+(molf/(v_a+v_b)).log(10.0) 		//excess of base. Strong base.
			}else{7.0													//Neutralisation. ph 7.0
		}
	}else if b.use_weak==false{  //weak acid, strong base.
		p_h= if (mol_h-mol_oh)/mol_oh>=10.0 {0.5*(a.pka[0].0-(molf/(v_a+v_b)).log(10.0))    //weak acid formula, big excess of acid.
			}else if ((mol_h-mol_oh)/mol_oh>=0.1) & ((mol_h-mol_oh)/mol_oh<10.0) {a.pka[0].0+(mol_oh/molf).log(10.0)  //buffer. small excess of weak acid.
			}else if (molf/mol_oh)<0.1 {7.0+0.5*(a.pka[0].0+(molf/(v_a+v_b)).log(10.0))		//weak base formula, roughly complete neutralisation.
			}else if -0.1>=(mol_h-mol_oh)/mol_oh {14.0+(molf/(v_a+v_b)).log(10.0)		//strong base formula, excess of base.
			}else{7.0																	//Just in case 7.0?
		}
	}else{    //weak base, strong acid (see base generator to see why).
		p_h= if (mol_oh-mol_h)/mol_h>=10.0 {0.5*(7.0+b.pka[0].0-(molf/(v_a+v_b)).log(10.0))    //weak base formula, big excess of base.
			}else if ((mol_oh-mol_h)/mol_h>=0.1) & ((mol_oh-mol_h)/mol_h<10.0) {b.pka[0].0+(molf/mol_h).log(10.0)  //buffer. small excess of weak base.
			}else if (molf/mol_h)<0.1 {0.5*(b.pka[0].0+(molf/(v_a+v_b)).log(10.0))		//weak acid formula, roughly complete neutralisation.
			}else if -0.1>=(mol_oh-mol_h)/mol_h {-(molf/(v_a+v_b)).log(10.0)		//strong acid formula, excess of strong acid.
			}else{7.0																	//Just in case 7.0?
		}
	};
		
	
	//Print Question.
	let question = format!("{}L of {}mol/L {} is added to {}L of {}mol/L {}. \
		 What is the pH of the resulting solution?",
		 dis(v_a),
		 dis(c_a),
		 a.name[0],
		 dis(v_b),
		 dis(c_b),
		 b.name[0]);
	
	//Print Answer. (name,acid/acid,x/c)
	let mut ans_a=Vec::new();
	ans_a.push(format!("Total volume = {} mL",(v_a+v_b)*1000.0));
	if a.use_weak==true{
		ans_a.push(format!("Acid (weak): {} -> {}{} + {}{}",a.formula[0],a.solutes[0].0,a.solutes[0].1,a.solutes[1].0,a.solutes[1].1))
	}else{
	ans_a.push(format!("Acid (strong): {} -> {}{} + {}{}",a.formula[0],a.solutes[0].0,a.solutes[0].1,a.solutes[1].0,a.solutes[1].1))
	};
	if b.use_weak==true{
	ans_a.push(format!("Base (weak): {} -> {}{} + {}{}",b.formula[0],b.solutes[0].0,b.solutes[0].1,b.solutes[1].0,b.solutes[1].1))
	}else{
	ans_a.push(format!("Base (strong): {} -> {}{} + {}{}",b.formula[0],b.solutes[0].0,b.solutes[0].1,b.solutes[1].0,b.solutes[1].1))
	};

	ans_a.push(format!("Moles H(+): {}n{}",a_bron.2,a.formula[0]));
	ans_a.push(format!("Moles {}({}-): {}n{}",b_bron.0,b_bron.1,b_bron.2,b.formula[0]));
	
	if b.use_weak==false{ //For strong base and weak or strong acid reaction.
		if (mol_h>mol_oh) & (a.use_weak==false) {
			ans_a.push(format!("Excess of strong acid: Use pH = -log[H+]"))
		}else if mol_oh>mol_h {
			ans_a.push(format!("Excess of strong base: Use pH = 14+log({}[{}({}-)])",b_bron.1,b_bron.0,b_bron.1))
		}else if ((mol_h-mol_oh)/mol_oh>=10.0) & (a.use_weak==true) {
			ans_a.push(format!("Large excess of weak acid: Use pH = 0.5 x (pKa-log({} x [{}])",a_bron.2,a.formula[0]))
		}else if ((mol_h-mol_oh)/mol_oh<10.0)
			& ((mol_h-mol_oh)/mol_oh>=0.1) 
			& (a.use_weak==true) {
				ans_a.push(format!("Small excess of weak acid (buffer!): Use pH = pKa + log([S]/[A])"))
		}else if ((mol_h-mol_oh)/mol_oh<0.1)
			& ((mol_h-mol_oh)/mol_oh>=-0.1) 
			& (a.use_weak==true) {
				ans_a.push(format!("Near perfect neutralisation of weak acid (calculate as salt of weak base): \
									Use pH = 7 + 0.5 x (pKa + log({} x [{}])",b_bron.1,b.formula[0]))
		}else{
			ans_a.push(format!("Perfect neutralisation. pH = 7"))};
	}else{ //NB assumes that acid is a strong acid (see base generator for why).
		if mol_h>mol_oh {
			ans_a.push(format!("Excess of strong acid: Use pH = -log[H+]"))
		}else if (mol_oh-mol_h)/mol_h>=10.0 {
			ans_a.push(format!("Large excess of weak base: Use pH = 7.0 + 0.5 x (pKa + log({} x [{}])",b_bron.2,b.formula[0]))
		}else if ((mol_oh-mol_h)/mol_h<10.0)
			& ((mol_oh-mol_h)/mol_h>=0.1) {
				ans_a.push(format!("Small excess of weak base (buffer!): Use pH = pKa + log([S]/[A])"))
		}else if ((mol_oh-mol_h)/mol_h<0.1)
			& ((mol_oh-mol_h)/mol_h>=-0.1) {
				ans_a.push(format!("Near perfect neutralisation of weak base (calculate as salt of weak acid): \
									Use pH = 0.5 x (pKa + log({} x [{}])",a_bron.1,a.formula[0]))
		}else{
			ans_a.push(format!("Perfect neutralisation. pH = 7"))};
	};
		
		
	let ans_b=format!("{}",format!("Answer = {}",&ff(4,p_h)));
	let answer = format!("{}\n\n {}\n",ans_a.join("\n"),ans_b);
	(question,answer)
}


//THIS FUNCTION SHOULD GIVE THE RIGHT ANSWER.	

pub fn q_6_2b(compounds:&Vec<Compound>)->(String,String){
//Reaction between strong acids and bases. Mass based.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;

	//set bronsted acid & base. (name,x/acid,x/c)
	let mut a_bron:(&str,u8,u8)=("H",1,1);
	let mut b_bron:(&str,u8,u8)=("OH",1,1);
	
	//Generate acid.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		let mut acid=false;
		//let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		if ((gives_h==true) & (x.pka[0].0<6.0))
		|| ((gives_h==false) & (x.salt==true) & (x.use_weak==true) & (x.pka[0].0>8.0)){acid=true}else{};
		if (x.solutes.len()==2)
		 & (acid==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	let strong_acid=if a.use_weak==true {false}else{true};
	
	
	//Generate base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut base=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if (y.1=="HCO3")||(y.1=="OH")||(y.1=="CO3"){gives_oh=true}else{}
		};
		if ((gives_oh==true) & (x.pka[0].0>8.0))
		|| ((gives_oh==false) & (x.salt==true) & (x.use_weak==true)& (x.pka[0].0<6.0)){base=true}else{};
		if (base==true)
		 & ((x.use_weak==false)||(strong_acid==true)){
			valid_c.push(&x)
		}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let b=&valid_c[indx];
	
	//determine bronsted acid (name,acid/acid,x/c)
	
	if a.salt==false{
		for x in a.solutes.iter(){
				if x.1=="H" {a_bron=("H",1,x.0)}else{}
		};
	}else{
		for x in a.solutes.iter(){
				if x.1==a.pka[0].1 {a_bron=("H",1,x.0)}else{}
		};
	};
	
	//determine bronsted base (name,acid/base,x/c)
	for x in b.solutes.iter(){
		let aob=(abs(x.2)) as u8;
		if (x.1=="OH")||(x.1=="HCO3")||(x.1=="CO3")||((x.1==b.pka[0].1) & b.salt & b.use_weak){b_bron=(&x.1,aob,x.0)}else{}
	};
	
	//generate concentration.
	let mut m_a:f64=(rand::thread_rng().gen_range(25,251) as f64)/500.0;
	let mut m_b:f64=(rand::thread_rng().gen_range(25,251) as f64)/500.0;
	
	//generate volume of acid (v_a) and base (v_b).
	let v_a=(rand::thread_rng().gen_range(30,1201) as f64)/1000.0;
	let v_b=(rand::thread_rng().gen_range(30,1201) as f64)/1000.0;
	
	//Some bases are almost insoluble so this reduces acid concentration to match:
	let mut silly=true;
	while silly==true{
		if b.solubility==f64::INFINITY{silly=false; continue
		}else{
			if m_a/v_a/10.0>b.solubility{m_a=m_a/10.0}else{silly=false}
		}
	};
	
	//Extra decimal space removal post solubility check.
	let m_a:f64= match ff(4,m_a).trim().parse(){
		Ok(num)=>num,
		Err(_)	 =>m_a,
	};
	
	//Check solubility of base.
	let mut silly=true;
	while silly==true{
		if b.solubility==f64::INFINITY{silly=false; continue
		}else{
			if m_b/v_b/10.0>b.solubility{m_b=m_b/10.0}else{silly=false}
		}
	};
	
	//Extra decimal space removal post solubility check.
	let m_b:f64= match ff(4,m_b).trim().parse(){
		Ok(num)=>num,
		Err(_)	 =>m_b,
	};
	
	//generate mass of acid (m_a) and base (m_b).
	let c_a=m_a/a.mmass/v_a;
	let c_b=m_b/b.mmass/v_b;
	
	//calculate moles H+ and B-
	let mol_h=c_a*(a_bron.2 as f64)*v_a;
	let mol_oh=c_b*((b_bron.2*b_bron.1) as f64)*v_b;
	
	//calculate moles remaining.
	let molf=absf64(mol_h-mol_oh);
	
	//get final pH
	let p_h;
	if (a.use_weak==false) & (b.use_weak==false){  //strong acid, strong base.
		p_h= if mol_h>mol_oh{-(molf/(v_a+v_b)).log(10.0)				//excess of acid. Strong acid.
			}else if mol_h<mol_oh {14.0+(molf/(v_a+v_b)).log(10.0) 		//excess of base. Strong base.
			}else{7.0													//Neutralisation. ph 7.0
		}
	}else if b.use_weak==false{  //weak acid, strong base.
		p_h= if (mol_h-mol_oh)/mol_oh>=10.0 {0.5*(a.pka[0].0-(molf/(v_a+v_b)).log(10.0))    //weak acid formula, big excess of acid.
			}else if ((mol_h-mol_oh)/mol_oh>=0.1) & ((mol_h-mol_oh)/mol_oh<10.0) {a.pka[0].0+(mol_oh/molf).log(10.0)  //buffer. small excess of weak acid.
			}else if (molf/mol_oh)<0.1 {7.0+0.5*(a.pka[0].0+(molf/(v_a+v_b)).log(10.0))		//weak base formula, roughly complete neutralisation.
			}else if -0.1>=(mol_h-mol_oh)/mol_oh {14.0+(molf/(v_a+v_b)).log(10.0)		//strong base formula, excess of base.
			}else{7.0																	//Just in case 7.0?
		}
	}else{    //weak base, strong acid (see base generator to see why).
		p_h= if (mol_oh-mol_h)/mol_h>=10.0 {0.5*(7.0+b.pka[0].0-(molf/(v_a+v_b)).log(10.0))    //weak base formula, big excess of base.
			}else if ((mol_oh-mol_h)/mol_h>=0.1) & ((mol_oh-mol_h)/mol_h<10.0) {b.pka[0].0+(molf/mol_h).log(10.0)  //buffer. small excess of weak base.
			}else if (molf/mol_h)<0.1 {0.5*(b.pka[0].0+(molf/(v_a+v_b)).log(10.0))		//weak acid formula, roughly complete neutralisation.
			}else if -0.1>=(mol_oh-mol_h)/mol_h {-(molf/(v_a+v_b)).log(10.0)		//strong acid formula, excess of strong acid.
			}else{7.0																	//Just in case 7.0?
		}
	};
		
	
	//Print Question.
	let mut question = Vec::new();
	question.push(format!("{}L of {} solution contains {}g of {}.",dis(v_a),a.name[0],dis(m_a),a.name[0]));
	question.push(format!("{}L of {} solution contains {}g of {}.",dis(v_b),b.name[0],dis(m_b),b.name[0]));
	question.push(format!("What is the pH of the resulting solution when these two initial solutions are mixed?"));
	let question = question.join("\n");
	
	//Print Answer. (name,acid/acid,x/c)
	let mut ans_a=Vec::new();
	ans_a.push(format!("[{}] = {} mol/L",a.formula[0],c_a));
	ans_a.push(format!("[{}] = {} mol/L",b.formula[0],c_b));
	ans_a.push(format!("Total volume = {} mL",(v_a+v_b)*1000.0));
	if a.use_weak==true{
		ans_a.push(format!("Acid (weak): {} -> {}{} + {}{}",a.formula[0],a.solutes[0].0,a.solutes[0].1,a.solutes[1].0,a.solutes[1].1))
	}else{
		ans_a.push(format!("Acid (strong): {} -> {}{} + {}{}",a.formula[0],a.solutes[0].0,a.solutes[0].1,a.solutes[1].0,a.solutes[1].1))
	};
	if b.use_weak==true{
		ans_a.push(format!("Base (weak): {} -> {}{} + {}{}",b.formula[0],b.solutes[0].0,b.solutes[0].1,b.solutes[1].0,b.solutes[1].1))
	}else{
		ans_a.push(format!("Base (strong): {} -> {}{} + {}{}",b.formula[0],b.solutes[0].0,b.solutes[0].1,b.solutes[1].0,b.solutes[1].1))
	};

	ans_a.push(format!("Moles H(+): {}n{}",a_bron.2,a.formula[0]));
	ans_a.push(format!("Moles {}({}-): {}n{}",b_bron.0,b_bron.1,b_bron.2,b.formula[0]));
	
	if b.use_weak==false{ //For strong base and weak or strong acid reaction.
		if (mol_h>mol_oh) & (a.use_weak==false) {
			ans_a.push(format!("Excess of strong acid: Use pH = -log[H+]"))
		}else if mol_oh>mol_h {
			ans_a.push(format!("Excess of strong base: Use pH = 14+log({}[{}({}-)])",b_bron.1,b_bron.0,b_bron.1))
		}else if ((mol_h-mol_oh)/mol_oh>=10.0) & (a.use_weak==true) {
			ans_a.push(format!("Large excess of weak acid: Use pH = 0.5 x (pKa-log({} x [{}])",a_bron.2,a.formula[0]))
		}else if ((mol_h-mol_oh)/mol_oh<10.0)
			& ((mol_h-mol_oh)/mol_oh>=0.1) 
			& (a.use_weak==true) {
				ans_a.push(format!("Small excess of weak acid (buffer!): Use pH = pKa + log([S]/[A])"))
		}else if ((mol_h-mol_oh)/mol_oh<0.1)
			& ((mol_h-mol_oh)/mol_oh>=-0.1) 
			& (a.use_weak==true) {
				ans_a.push(format!("Near perfect neutralisation of weak acid (calculate as salt of weak base): \
									Use pH = 7 + 0.5 x (pKa + log({} x [{}])",b_bron.1,b.formula[0]))
		}else{
			ans_a.push(format!("Perfect neutralisation. pH = 7"))
		};
	}else{ //NB assumes that acid is a strong acid (see base generator for why).
		if mol_h>mol_oh {
			ans_a.push(format!("Excess of strong acid: Use pH = -log[H+]"))
		}else if (mol_oh-mol_h)/mol_h>=10.0 {
			ans_a.push(format!("Large excess of weak base: Use pH = 7.0 + 0.5 x (pKa + log({} x [{}])",b_bron.2,b.formula[0]))
		}else if ((mol_oh-mol_h)/mol_h<10.0)
			& ((mol_oh-mol_h)/mol_h>=0.1) {
				ans_a.push(format!("Small excess of weak base (buffer!): Use pH = pKa + log([S]/[A])"))
		}else if ((mol_oh-mol_h)/mol_h<0.1)
			& ((mol_oh-mol_h)/mol_h>=-0.1) {
				ans_a.push(format!("Near perfect neutralisation of weak base (calculate as salt of weak acid): \
									Use pH = 0.5 x (pKa + log({} x [{}])",a_bron.1,a.formula[0]))
		}else{
			ans_a.push(format!("Perfect neutralisation. pH = 7"))
		};
	};
		
		
	let ans_b=format!("{}",format!("Answer = {}",&ff(4,p_h)));
	let answer = format!("{}\n\n {}\n",ans_a.join("\n"),ans_b);
	(question,answer)	
}


pub fn q_6_3(compounds:&Vec<Compound>)->(String,String){
//Find degree of ionisation from concentration.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	
	//Generate strong acid or base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut useable=true;
		for y in x.solutes.iter(){
			if y.0>1{useable=false}else{useable=useable}
		};
		if (x.solutes.len()==2)
		 & (x.pka[0].0<6.0)
		 & (useable==true)
		 & (x.salt==false)
		 & (x.use_weak==true) {valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	//generate concentration.
	let conc_high:f64=(rand::thread_rng().gen_range(1,670) as f64)/5000.0;
	let conc_low:f64=(rand::thread_rng().gen_range(1,670) as f64)/5000000.0;
	
	let dice=rand::thread_rng().gen_range(0,10);
	let mut conc=if dice<5 {conc_high}else{conc_low};
		
	//Check solubility
	let mut silly=true;
	while silly==true{
		if c.solubility==f64::INFINITY{silly=false; continue
		}else{
			if conc*c.mmass/10.0>c.solubility{conc=conc/10.0}else{silly=false}
		}
	};	
	
	let ion_exact=(TEN.powf(-c.pka[0].0)*(conc+TEN.powf(-c.pka[0].0)*0.25)).sqrt()-TEN.powf(-c.pka[0].0)*0.5;
	let ion_approx=(conc*TEN.powf(-c.pka[0].0)).sqrt();
	
	let ans_exact=ion_exact/conc*100.0;
	let ans_approx=ion_approx/conc*100.0;
	
	let question = format!("What is the degree of ionisation of {}mol/L {} (as a percentage):\n\
	(Use the approximation, the exact method, or both if you wish!)",
		dis(conc),
		c.name[0]);
	
	let mut ans_a=Vec::new();
	if ans_approx<100.0{
		ans_a.push(format!("Approximate degree of ionisation: {}%\n",ff(4,ans_approx)))
	}else{
		ans_a.push(format!("Approximate degree of ionisation: {}%\n(This method gives us a silly answer!)\n",ff(4,ans_approx)))
	};
	
	let ans_b=format!("{}\n",format!("Exact degree of ionisation: {}%\n", ff(4,ans_exact)));
	let answer = format!("{}\n{}",ans_a.join(""),ans_b);
	(question,answer)	
}


pub fn q_6_3b(compounds:&Vec<Compound>)->(String,String){
//Find concentration from degree of ionisation.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;

	//Generate strong acid or base.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut useable=true;
		for y in x.solutes.iter(){
			if y.0>1{useable=false}else{useable=useable}
		};
		if (x.solutes.len()==2)
		 & (x.pka[0].0<6.0)
		 & (useable==true)
		 & (x.salt==false)
		 & (x.use_weak==true) {valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let c=&valid_c[indx];
	
	//generate concentration.
	let degree:f64=(rand::thread_rng().gen_range(10,670) as f64)/10.0;
		
	//Check solubility... Can't can I?
	
	//Generate answer.
	let deg=degree*0.01;
	
	let conc_exact=TEN.powf(-c.pka[0].0)*(1.0-deg)/deg/deg;
	let conc_approx=TEN.powf(-c.pka[0].0)/deg;
	
	let exact_prefix= if conc_exact<0.0001 {"μ"}else if conc_exact<0.1 {"m"}else{""};
	let approx_prefix= if conc_approx<0.0001 {"μ"}else if conc_approx<0.1 {"m"}else{""};
	let ans_approx= if conc_approx<0.0001 {
						conc_approx*1000000.0
					}else if conc_approx<0.1 {
						conc_approx*1000.0
					}else{conc_approx
	};	
	let ans_exact= if conc_exact<0.0001 {
						conc_exact*1000000.0
					}else if conc_exact<0.1 {
						conc_exact*1000.0
					}else{conc_exact
	};
	
	//PRINT QUESTION.
	let question = format!("If the degree of ionisation of a {} solution is {}% then what is its concentration:\n\
	(By approximate method? By exact method?)",
	c.name[0],degree);
	
	//PRINT ANSWER.
	let ans_a=format!("Concentration by approximate method: {} {}mol/L",ff(4,ans_approx),approx_prefix);
	let ans_b=format!("{}",format!("Concentration by exact method: {} {}mol/L", ff(4,ans_exact),exact_prefix));	
	let answer = format!("{}\n\n{}\n",ans_a,ans_b);
	(question,answer)	
}
//Buffers
//Buffers
//Buffers
//Buffers

pub fn q_7_0(compounds:&Vec<Compound>)->(String,String){
//pH as function of concs.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	
	//Generate acid or base.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		//let mut acid=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		for y in x.solutes.iter(){
			if y.1=="OH" {gives_oh=true}else{}
		};
		if (((gives_h==true) & (x.pka[0].0<6.0)) || ((gives_oh==true) & (x.pka[0].0>8.0)))
		& (x.salt==false)
		& (x.use_weak==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	
	//is compound A an acid (important!)
	let a_is_acid=if a.pka[0].0<7.0 {true}else{false};
		
	//Generate salt.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut salt_of_a=false;
		for y in x.solutes.iter(){
			if y.1==a.pka[0].1 {salt_of_a=true}else{}
		};
		salt_of_a= if x.salt==false{false}else{salt_of_a};
		if (((a_is_acid==true) & (x.pka[0].0<6.0)) || ((a_is_acid==false) & (x.pka[0].0>8.0)))
		& (x.use_weak==true)
		& (salt_of_a==true){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let s=&valid_c[indx];
	
	//generate concentration of b. (nb salt is usually les soluble so solubility check is here)
	let mut s_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check solubility
	let mut silly=true;
	while silly==true{
		if s.solubility==f64::INFINITY{silly=false; continue
		}else{
			if s_c*s.mmass/10.0>s.solubility{s_c=s_c/10.0}else{silly=false}
		}
	};
	
	//concentration of ions in salt. Kind of.
	let mut nimp_in_s=0.0;
		for y in s.solutes.iter(){
			if y.1==s.pka[0].1 {nimp_in_s=y.0 as f64}else{}
	};
	
	let mut a_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check whether this makes a buffer question.
	let mut silly=true;
	while silly==true{	
		if (a_c*10.0)<s_c*nimp_in_s {s_c=s_c*0.1
		}else if s_c*nimp_in_s<(a_c*0.1) {a_c=a_c*0.1
		}else{silly=false}
	};
	
	//concentration of active ion in A and B
	let mut nimp_in_a=0.0;
	for y in a.solutes.iter(){
			if y.1==a.pka[0].1 {nimp_in_a=y.0 as f64}else{}
	};
	let ion_in_a= a_c*nimp_in_a;
	let ion_in_s= s_c*nimp_in_s;
	
	
	let c_base= if a.pka[0].0<7.0{ion_in_s}else{ion_in_a};
	let	c_acid= if a.pka[0].0<7.0{ion_in_a}else{ion_in_s};
	
	let p_h=a.pka[0].0+(c_base/c_acid).log(10.0);
	let coin_toss=rand::thread_rng().gen_range(0,10);
	
	let mut c_1=a_c;
	let mut c_2=s_c;
	let mut n_1=&a.name[0];
	let mut n_2=&s.name[0];
	if coin_toss<5{}else{
		c_1=s_c;
		c_2=a_c;
		n_1=&s.name[0];
		n_2=&a.name[0]
	};	
	
	let question = format!("A buffer contains {}mol/L {} and {}mol/L {}. What is its pH?",
	dis(c_1),
	n_1,
	dis(c_2),
	n_2);
	
	let mut ans_a=Vec::new();
	ans_a.push(format!("Henderson equation!"));
	if a.pka[0].0<7.0{
		ans_a.push(format!("\npH = pKa + log([S]/[A])"));
		ans_a.push(format!("\npH = pKa + log([{} x {}]/[{} x {}])",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
	}else{
		ans_a.push(format!("\npH = pKa + log([B]/[S])"));
		ans_a.push(format!("\npH = pKa + log([{} x {}]/[{} x {}])",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));		
	};
	
	let ans_b=format!("{}",format!("Answer = {}",&ff(4,p_h)));
	let answer = format!("{}\n\n {}\n",ans_a.join(""),ans_b);
	(question,answer)
}	


pub fn q_7_0b(compounds:&Vec<Compound>)->(String,String){
//pH as function of compounds' volumes and concentrations.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	
	//Generate acid or base.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		//let mut acid=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		for y in x.solutes.iter(){
			if y.1=="OH" {gives_oh=true}else{}
		};
		if (((gives_h==true) & (x.pka[0].0<6.0)) || ((gives_oh==true) & (x.pka[0].0>8.0)))
		& (x.salt==false)
		& (x.use_weak==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	
	//is compound A an acid (important!)
	let a_is_acid=if a.pka[0].0<7.0 {true}else{false};
		
	//Generate salt.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut salt_of_a=false;
		for y in x.solutes.iter(){
			if y.1==a.pka[0].1 {salt_of_a=true}else{}
		};
		salt_of_a= if x.salt==false{false}else{salt_of_a};
		if (((a_is_acid==true) & (x.pka[0].0<6.0)) || ((a_is_acid==false) & (x.pka[0].0>8.0)))
		& (x.use_weak==true)
		& (salt_of_a==true){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let s=&valid_c[indx];
	
	//generate concentration of b. (nb salt is usually les soluble so solubility check is here)
	let s_vol=(rand::thread_rng().gen_range(100,1001) as f64)/500.0;
	let mut s_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check solubility
	let mut silly=true;
	while silly==true{
		if s.solubility==f64::INFINITY{silly=false; continue
		}else{
			if s_c*s.mmass/s_vol*0.1>s.solubility{s_c=s_c/10.0}else{silly=false}
		}
	};
	
	//ratios of ions in A and B. Kind of.
	let mut nimp_in_s=0.0;
		for y in s.solutes.iter(){
			if y.1==s.pka[0].1 {nimp_in_s=y.0 as f64}else{}
	};
	let mut nimp_in_a=0.0;
	for y in a.solutes.iter(){
			if y.1==a.pka[0].1 {nimp_in_a=y.0 as f64}else{}
	};
	
	let mut a_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	let a_vol=(rand::thread_rng().gen_range(100,1001) as f64)/500.0;
	
	//check whether this makes a buffer question.
	let mut silly=true;
	while silly==true{	
		if (a_c*a_vol*nimp_in_a*10.0)<(s_c*s_vol*nimp_in_s) {
			s_c=s_c*0.5;
		}else if (s_c*s_vol*nimp_in_s)<(a_c*a_vol*0.1) {a_c=a_c*0.1
		}else{silly=false}
	};
	let a_mol=a_c*a_vol;
	let s_mol=s_c*s_vol;
	
	//concentration of active ion in A and B
	let ion_in_a= a_mol*nimp_in_a;
	let ion_in_s= s_mol*nimp_in_s;	
	
	let c_base= if a.pka[0].0<7.0{ion_in_s}else{ion_in_a};
	let	c_acid= if a.pka[0].0<7.0{ion_in_a}else{ion_in_s};
	
	let p_h=a.pka[0].0+(c_base/c_acid).log(10.0);
	let coin_toss=rand::thread_rng().gen_range(0,10);
	
	let mut c_1=a_c;
	let mut c_2=s_c;
	let mut v_1=a_vol;
	let mut v_2=s_vol;
	let mut n_1=&a.name[0];
	let mut n_2=&s.name[0];
	if coin_toss<5{}else{
		c_1=s_c;
		c_2=a_c;
		v_1=s_vol;
		v_2=a_vol;
		n_1=&s.name[0];
		n_2=&a.name[0]
	};	

	let question = format!("{}L of {}M solution of {} is mixed with {}L of {}M {} solution. What is the final pH?",
	dis(v_1),
	dis(c_1),
	n_1,
	dis(v_2),
	dis(c_2),
	n_2);
	
	let mut ans_a=Vec::new();
	ans_a.push(format!("Henderson equation!"));
	if a.pka[0].0<7.0{
		ans_a.push(format!("\npH = pKa + log(nS/nA)"));
		ans_a.push(format!("\npH = pKa + log({} x {} / {} x {})",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		ans_a.push(format!("\nMoles salt: {}\nMoles acid: {}",s_mol,a_mol));
	}else{
		ans_a.push(format!("\npH = pKa + log(nB/nS)"));
		ans_a.push(format!("\npH = pKa + log({} x {} / {} x {})",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));	
		ans_a.push(format!("\nMoles base: {}\nMoles salt: {}",a_mol,s_mol));	
	};
	
	let ans_b=format!("{}",format!("Answer = {}",&ff(4,p_h)));
	let answer = format!("{}\n\n {}\n",ans_a.join(""),ans_b);
	(question,answer)
}	


pub fn q_7_0c(compounds:&Vec<Compound>)->(String,String){
//pH as function of compounds masses
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	
	//Generate acid or base.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		//let mut acid=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		for y in x.solutes.iter(){
			if y.1=="OH" {gives_oh=true}else{}
		};
		if (((gives_h==true) & (x.pka[0].0<6.0)) || ((gives_oh==true) & (x.pka[0].0>8.0)))
		& (x.salt==false)
		& (x.use_weak==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	
	//is compound A an acid (important!)
	let a_is_acid=if a.pka[0].0<7.0 {true}else{false};
		
	//Generate salt.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut salt_of_a=false;
		for y in x.solutes.iter(){
			if y.1==a.pka[0].1 {salt_of_a=true}else{}
		};
		salt_of_a= if x.salt==false{false}else{salt_of_a};
		if (((a_is_acid==true) & (x.pka[0].0<6.0)) || ((a_is_acid==false) & (x.pka[0].0>8.0)))
		& (x.use_weak==true)
		& (salt_of_a==true){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let s=&valid_c[indx];
	
	//generate concentration of b. (nb salt is usually les soluble so solubility check is here)
	let s_vol=(rand::thread_rng().gen_range(100,1001) as f64)/500.0;
	let mut s_mass:f64=(rand::thread_rng().gen_range(10,670) as f64)/50.0;
	
	//check solubility
	let mut silly=true;
	while silly==true{
		if s.solubility==f64::INFINITY{silly=false; continue
		}else{
			if s_mass/s_vol*0.1>s.solubility{s_mass=s_mass/10.0}else{silly=false}
		}
	};
	
	//ratios of ions in A and B. Kind of.
	let mut nimp_in_s=0.0;
		for y in s.solutes.iter(){
			if y.1==s.pka[0].1 {nimp_in_s=y.0 as f64}else{}
	};
	let mut nimp_in_a=0.0;
	for y in a.solutes.iter(){
			if y.1==a.pka[0].1 {nimp_in_a=y.0 as f64}else{}
	};
	
	let mut a_mass:f64=(rand::thread_rng().gen_range(10,670) as f64)/50.0;
	let a_vol=(rand::thread_rng().gen_range(100,1001) as f64)/500.0;
	
	//check whether this makes a buffer question.
	let mut silly=true;
	while silly==true{	
		if (a_mass/a.mmass*nimp_in_a*10.0)<(s_mass/s.mmass*nimp_in_s) {
			s_mass=s_mass*0.1;
		}else if (s_mass/s.mmass*nimp_in_s)<(a_mass/a.mmass*0.1) {a_mass=a_mass*0.1
		}else{silly=false}
	};
	let a_mol=a_mass/a.mmass;
	let s_mol=s_mass/s.mmass;
	
	//concentration of active ion in A and B
	let ion_in_a= a_mol*nimp_in_a;
	let ion_in_s= s_mol*nimp_in_s;	
	
	let c_base= if a.pka[0].0<7.0{ion_in_s}else{ion_in_a};
	let	c_acid= if a.pka[0].0<7.0{ion_in_a}else{ion_in_s};
	
	let p_h=a.pka[0].0+(c_base/c_acid).log(10.0);
	let coin_toss=rand::thread_rng().gen_range(0,10);
	
	let mut c_1=a_mass;
	let mut c_2=s_mass;
	let mut v_1=a_vol;
	let mut v_2=s_vol;
	let mut n_1=&a.name[0];
	let mut n_2=&s.name[0];
	if coin_toss<5{}else{
		c_1=s_mass;
		c_2=a_mass;
		v_1=s_vol;
		v_2=a_vol;
		n_1=&s.name[0];
		n_2=&a.name[0]
	};	
	
		let question = format!("A solution containing {}g of {} in {}L is mixed with {}L of solution containing {}g of {}. What is the final pH?",
		dis(c_1),
		n_1,
		dis(v_1),
		dis(v_2),
		dis(c_2),
		n_2);
	
	let mut ans_a=Vec::new();
	ans_a.push(format!("Henderson equation!"));
	if a.pka[0].0<7.0{
		ans_a.push(format!("\npH = pKa + log(nS/nA)"));
		ans_a.push(format!("\npH = pKa + log({} x {} / {} x {})",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		ans_a.push(format!("\nMoles salt: {}\nMoles acid: {}",s_mol,a_mol));
	}else{
		ans_a.push(format!("\npH = pKa + log(nB/nS)"));
		ans_a.push(format!("\npH = pKa + log({} x {} / {} x {})",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));	
		ans_a.push(format!("\nMoles base: {}\nMoles salt: {}",a_mol,s_mol));	
	};
	
	let ans_b=format!("{}",format!("Answer = {}",&ff(4,p_h)));
	let answer = format!("{}\n\n {}\n",ans_a.join(""),ans_b);
	(question,answer)
}	
	
	

pub fn q_7_1(compounds:&Vec<Compound>)->(String,String){
//Concs as function of pH.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	
	//Generate acid or base.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		//let mut acid=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		for y in x.solutes.iter(){
			if y.1=="OH" {gives_oh=true}else{}
		};
		if (((gives_h==true) & (x.pka[0].0<6.0)) || ((gives_oh==true) & (x.pka[0].0>8.0)))
		& (x.salt==false)
		& (x.use_weak==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	
	//is compound A an acid (important!)
	let a_is_acid=if a.pka[0].0<7.0 {true}else{false};
		
	//Generate salt.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut salt_of_a=false;
		for y in x.solutes.iter(){
			if y.1==a.pka[0].1 {salt_of_a=true}else{}
		};
		salt_of_a= if x.salt==false{false}else{salt_of_a};
		if (((a_is_acid==true) & (x.pka[0].0<6.0)) || ((a_is_acid==false) & (x.pka[0].0>8.0)))
		& (x.use_weak==true)
		& (salt_of_a==true){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let s=&valid_c[indx];
	
	//generate concentration of b. (nb salt is usually les soluble so solubility check is here)
	let mut s_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check solubility
	let mut silly=true;
	while silly==true{
		if s.solubility==f64::INFINITY{silly=false; continue
		}else{
			if s_c*s.mmass/10.0>s.solubility{s_c=s_c/10.0}else{silly=false}
		}
	};
	
	//concentration of ions in salt. Kind of.
	let mut nimp_in_s=0.0;
		for y in s.solutes.iter(){
			if y.1==s.pka[0].1 {nimp_in_s=y.0 as f64}else{}
	};
	
	let mut a_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check whether this makes a buffer question.
	let mut silly=true;
	while silly==true{	
		if (a_c*10.0)<s_c*nimp_in_s {s_c=s_c*0.1
		}else if s_c*nimp_in_s<(a_c*0.1) {a_c=a_c*0.1
		}else{silly=false}
	};
	
	//concentration of active ion in A and B
	let mut nimp_in_a=0.0;
	for y in a.solutes.iter(){
			if y.1==a.pka[0].1 {nimp_in_a=y.0 as f64}else{}
	};
	let ion_in_a= a_c*nimp_in_a;
	let ion_in_s= s_c*nimp_in_s;
	
	
	let c_base= if a.pka[0].0<7.0{ion_in_s}else{ion_in_a};
	let	c_acid= if a.pka[0].0<7.0{ion_in_a}else{ion_in_s};
	
	let p_h=a.pka[0].0+(c_base/c_acid).log(10.0);
	let coin_toss=rand::thread_rng().gen_range(0,10);
	
	let mut c_1=a_c;
	let mut c_2=s_c;
	let mut n_1=&a.name[0];
	let mut n_2=&s.name[0];
	if coin_toss<5{}else{
		c_1=s_c;
		c_2=a_c;
		n_1=&s.name[0];
		n_2=&a.name[0]
	};	
	
	
	let question = format!("A {} / {} buffer contains {}mol/L {} and has a pH of {}. What is the concentration of {}?",
		n_1,
		n_2,
		dis(c_1),
		n_1,
		ff(4,p_h),
		n_2);
	
	let mut ans_a=Vec::new();
	ans_a.push(format!("Henderson equation!"));
	if a.pka[0].0<7.0{
		ans_a.push(format!("\npH = pKa + log([S]/[A])"));
		ans_a.push(format!("\npH = pKa + log([{} x {}]/[{} x {}])",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		if coin_toss<5 {
			ans_a.push(format!("\n[{} x {}] = [{} x {}] x 10^(pH-pKa)",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		}else{
			ans_a.push(format!("\n[{} x {}] = [{} x {}] x 10^(pKa-pH)",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));
		}
	}else{
		ans_a.push(format!("\npH = pKa + log([B]/[S])"));
		ans_a.push(format!("\npH = pKa + log([{} x {}]/[{} x {}])",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));	
		if coin_toss<5 {
			ans_a.push(format!("\n[{} x {}] = [{} x {}] x 10^(pH-pKa)",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));
		}else{
			ans_a.push(format!("\n[{} x {}] = [{} x {}] x 10^(pKa-pH)",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		}	
	};

	let ans_b=format!("{}",format!("Answer = {}mol/L",dis(c_2)));
	let answer = format!("{}\n\n {}\n",ans_a.join(""),ans_b);
	(question,answer)
}


pub fn q_7_1b(compounds:&Vec<Compound>)->(String,String){
//pH as function of compounds' volumes and concentrations. In reverse, get volume.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	
	//Generate acid or base.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		//let mut acid=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		for y in x.solutes.iter(){
			if y.1=="OH" {gives_oh=true}else{}
		};
		if (((gives_h==true) & (x.pka[0].0<6.0)) || ((gives_oh==true) & (x.pka[0].0>8.0)))
		& (x.salt==false)
		& (x.use_weak==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	
	//is compound A an acid (important!)
	let a_is_acid=if a.pka[0].0<7.0 {true}else{false};
		
	//Generate salt.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut salt_of_a=false;
		for y in x.solutes.iter(){
			if y.1==a.pka[0].1 {salt_of_a=true}else{}
		};
		salt_of_a= if x.salt==false{false}else{salt_of_a};
		if (((a_is_acid==true) & (x.pka[0].0<6.0)) || ((a_is_acid==false) & (x.pka[0].0>8.0)))
		& (x.use_weak==true)
		& (salt_of_a==true){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let s=&valid_c[indx];
	
	//generate concentration of b. (nb salt is usually les soluble so solubility check is here)
	let s_vol=(rand::thread_rng().gen_range(100,1001) as f64)/500.0;
	let mut s_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check solubility
	let mut silly=true;
	while silly==true{
		if s.solubility==f64::INFINITY{silly=false; continue
		}else{
			if s_c*s.mmass/s_vol*0.1>s.solubility{s_c=s_c/10.0}else{silly=false}
		}
	};
	
	//ratios of ions in A and B. Kind of.
	let mut nimp_in_s=0.0;
		for y in s.solutes.iter(){
			if y.1==s.pka[0].1 {nimp_in_s=y.0 as f64}else{}
	};
	let mut nimp_in_a=0.0;
	for y in a.solutes.iter(){
			if y.1==a.pka[0].1 {nimp_in_a=y.0 as f64}else{}
	};
	
	let mut a_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	let a_vol=(rand::thread_rng().gen_range(100,1001) as f64)/500.0;
	
	//check whether this makes a buffer question.
	let mut silly=true;
	while silly==true{	
		if (a_c*a_vol*nimp_in_a*10.0)<(s_c*s_vol*nimp_in_s) {
			s_c=s_c*0.1;
		}else if (s_c*s_vol*nimp_in_s)<(a_c*a_vol*0.1) {a_c=a_c*0.1
		}else{silly=false}
	};
	let a_mol=a_c*a_vol;
	let s_mol=s_c*s_vol;
	
	//concentration of active ion in A and B
	let ion_in_a= a_mol*nimp_in_a;
	let ion_in_s= s_mol*nimp_in_s;	
	
	let c_base= if a.pka[0].0<7.0{ion_in_s}else{ion_in_a};
	let	c_acid= if a.pka[0].0<7.0{ion_in_a}else{ion_in_s};
	
	let p_h=a.pka[0].0+(c_base/c_acid).log(10.0);
	let coin_toss=rand::thread_rng().gen_range(0,10);
	
	let mut c_1=a_c;
	let mut c_2=s_c;
	let mut v_1=a_vol;
	let mut v_2=s_vol;
	let mut n_1=&a.name[0];
	let mut n_2=&s.name[0];
	if coin_toss<5{}else{
		c_1=s_c;
		c_2=a_c;
		v_1=s_vol;
		v_2=a_vol;
		n_1=&s.name[0];
		n_2=&a.name[0]
	};	

	let question = format!("{}L of {}M solution of {} is mixed with {}M {} solution. The final pH is {}. What was the volume of the {} solution?",
		dis(v_1),
		dis(c_1),
		n_1,
		dis(c_2),
		n_2,
		ff(4,p_h),
		n_2);
	
	let mut ans_a=Vec::new();
	ans_a.push(format!("Henderson equation!"));
	if a.pka[0].0<7.0{
		ans_a.push(format!("\npH = pKa + log(nS/nA)"));
		ans_a.push(format!("\npH = pKa + log({} x {} / {} x {})",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		if coin_toss<5 {
			ans_a.push(format!("\nMoles acid: {} mol",a_mol));
			ans_a.push(format!("\n[{} x {}] = [{} x {}] x 10^(pKa-pH)",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
			
		}else{
			ans_a.push(format!("\nMoles salt: {} mol",s_mol));
			ans_a.push(format!("\n[{} x {}] = [{} x {}] x 10^(pH-pKa)",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));
		};	
		ans_a.push(format!("\nV=n/c"));
	}else{
		ans_a.push(format!("\npH = pKa + log(nB/nS)"));
		ans_a.push(format!("\npH = pKa + log({} x {} / {} x {})",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));	
		if coin_toss<5 {
			ans_a.push(format!("\nMoles base: {} mol",a_mol));
			ans_a.push(format!("\n[{} x {}] = [{} x {}] x 10^(pKa-pH)",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		}else{
			ans_a.push(format!("\nMoles salt: {} mol",s_mol));
			ans_a.push(format!("\n[{} x {}] = [{} x {}] x 10^(pH-pKa)",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));
		};	
		ans_a.push(format!("\nV=n/c"));
	};
	
	let ans_b=format!("{}",format!("Answer = {}L",dis(v_2)));
	let answer = format!("{}\n\n {}\n",ans_a.join(""),ans_b);
	(question,answer)
}	


pub fn q_7_2(compounds:&Vec<Compound>)->(String,String){
//Change in buffer pH after addition of strong acid/base.
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	
	//Generate acid or base.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		//let mut acid=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		for y in x.solutes.iter(){
			if y.1=="OH" {gives_oh=true}else{}
		};
		if (((gives_h==true) & (x.pka[0].0<6.0)) || ((gives_oh==true) & (x.pka[0].0>8.0)))
		& (x.salt==false)
		& (x.use_weak==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	
	//is compound A an acid (important!)
	let a_is_acid=if a.pka[0].0<7.0 {true}else{false};
		
	//Generate salt.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut salt_of_a=false;
		for y in x.solutes.iter(){
			if y.1==a.pka[0].1 {salt_of_a=true}else{}
		};
		salt_of_a= if x.salt==false{false}else{salt_of_a};
		if (((a_is_acid==true) & (x.pka[0].0<6.0)) || ((a_is_acid==false) & (x.pka[0].0>8.0)))
		& (x.use_weak==true)
		& (salt_of_a==true){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let s=&valid_c[indx];
	
	//Generate strong acid or base.
	let mut valid_aob:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut a_o_b=false;
		for y in x.solutes.iter(){
			if (y.1=="H")||(y.1=="OH")||(y.1=="HCO3")||(y.1=="CO3"){a_o_b=true}else{};
		};
		if (x.solutes.len()==2)
		 & (a_o_b==true)
		 & (x.use_weak==false)
		 & (x.salt==false)
		 & ((x.solubility>1.0)||(x.solubility==f64::INFINITY))
		 & (x.pka[0].0!=7.0){valid_aob.push(&x)}else{}
	};
	let vaob_len=valid_aob.len();
	let indx=rand::thread_rng().gen_range(0,vaob_len);
	let strong=&valid_aob[indx];
	
	//generate concentration.
	let mut strong_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/250.0;
	
	//Check solubility
	let mut silly=true;
	while silly==true{
		if strong.solubility==f64::INFINITY{silly=false; continue
		}else{
			if strong_c*strong.mmass/10.0>strong.solubility{strong_c=strong_c/10.0}else{silly=false}
		}
	};
	
	//generate concentration of b. (nb salt is usually less soluble so solubility check is here)
	let mut s_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check solubility
	let mut silly=true;
	while silly==true{
		if s.solubility==f64::INFINITY{silly=false; continue
		}else{
			if s_c*s.mmass/10.0>s.solubility{s_c=s_c/2.5}else{silly=false}
		}
	};
	
	let v_strong=rand::thread_rng().gen_range(50,671) as f64/1000.0;
	let v_buf=rand::thread_rng().gen_range(100,1341) as f64/1000.0;
	
	//concentration of ions in salt. Kind of.
	let mut nimp_in_s=0.0;
		for y in s.solutes.iter(){
			if y.1==s.pka[0].1 {nimp_in_s=y.0 as f64}else{}
	};
	
	let mut a_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check whether this makes a buffer question.
	let mut silly=true;
	while silly==true{	
		if (a_c*10.0)<s_c*nimp_in_s {s_c=s_c*0.1
		}else if s_c*nimp_in_s<(a_c*0.1) {a_c=a_c*0.1
		}else{silly=false}
	};
	
	//concentration of active ion in A and B
	let mut nimp_in_a=0.0;
	for y in a.solutes.iter(){
			if y.1==a.pka[0].1 {nimp_in_a=y.0 as f64}else{}
	};
	let ion_in_a= a_c*nimp_in_a*v_buf;
	let ion_in_s= s_c*nimp_in_s*v_buf;
	
	
	let n_base= if a.pka[0].0<7.0{ion_in_s}else{ion_in_a};
	let	n_acid= if a.pka[0].0<7.0{ion_in_a}else{ion_in_s};
	
	let p_h_start=a.pka[0].0+(n_base/n_acid).log(10.0);
	
	//effective concentration of storng acid or base.
	let mut nimp_in_strong=0.0;
	for y in strong.solutes.iter(){
			if (y.1=="H")||(y.1=="OH")||(y.1=="HCO3")||(y.1=="CO3"){
				nimp_in_strong=(y.0 as f64)*absf64(y.2 as f64)
			}else{}
	};
	
	let n_strong=nimp_in_strong*v_strong*strong_c;
	
	//is the strong stuff an acid or a base?
	let strong_is_acid= if strong.pka[0].0<7.0 {true}else{false};
	
	//final volume.
	let v_fin=v_buf+v_strong;
	
	//Generate answer.
	//let mut n_base_fin=n_base;
	//let mut n_acid_fin=n_acid;
	let mut p_h_fin=p_h_start;
	let mut marker:(String,String,String,char)=("acid".to_owned(),
												"buffer stable".to_owned(),
												"calculate new concentrations and use Henderson".to_owned(),
												'a');
	
	if (strong_is_acid==true) & ((n_base-n_strong)/(n_acid+n_strong)>0.1){
		marker=("acid".to_owned(),
				"buffer is stable".to_owned(),
				"calculate new concentrations and use Henderson".to_owned(),
				'a');
		p_h_fin=a.pka[0].0+((n_base-n_strong)/(n_acid+n_strong)).log(10.0)    	//acidified buffer.
	}else if (strong_is_acid==true) & ((n_base-n_strong)/(n_acid+n_strong)>=-0.1){
		marker=("acid".to_owned(),
				"buffer capacity is exceeded, no strong acid left".to_owned(),
				"calculate new concentrations and use weak acid equatiion".to_owned(),
				'b');
		p_h_fin=0.5*(a.pka[0].0-(n_acid/v_fin+n_base/v_fin).log(10.0))			//Weak acid on a Knifedge.
	}else if (strong_is_acid==true) & ((n_strong-n_base)/(n_acid+n_strong)>0.1){
		marker=("acid".to_owned(),
				"buffer capacity is exceeded, strong acid left over".to_owned(),
				"calculate new concentrations and use strong acid equation".to_owned(),
				'c');
		p_h_fin= -((n_strong-n_base)/v_fin).log(10.0)						//Overacidifed. Strong acid.
	}else if (strong_is_acid==false)
		   & (((n_base+n_strong)/(n_acid-n_strong)<10.0) & ((n_base+n_strong)/(n_acid-n_strong)>0.1)){
		marker=("base".to_owned(),
				"buffer is stable".to_owned(),
				"calculate new concentrations and use Henderson".to_owned(),
				'A');
		p_h_fin=a.pka[0].0+((n_base+n_strong)/(n_acid-n_strong)).log(10.0)		//alkalinised buffer.
	}else if (strong_is_acid==false)
	       & (((n_acid-n_strong)/(n_base+n_strong)<=0.1) & ((n_acid-n_strong)/(n_base+n_strong)>=-0.1)){
		marker=("base".to_owned(),
				"buffer capacity is exceeded, no strong base left".to_owned(),
				"calculate new concentrations and use weak base equation".to_owned(),
				'B');
		p_h_fin=7.0+0.5*(a.pka[0].0+(n_acid/v_fin+n_base/v_fin).log(10.0))		//Weak base on a knifedge.
	}else if (strong_is_acid==false) & ((n_strong-n_acid)/(n_base+n_strong)>0.1){
		marker=("base".to_owned(),
				"buffer capacity is exceeded, strong base left over".to_owned(),
				"calculate new concentrations and use strong base equation".to_owned(),
				'C');
		p_h_fin= 14.0+((n_strong-n_acid)/v_fin).log(10.0)						//Overalkanisied. Strong acid.
	}else{};
	
	//Prepare to print question.
	let coin_toss=rand::thread_rng().gen_range(0,10);
	
	let mut c_1=a_c;
	let mut c_2=s_c;
	let mut n_1=&a.name[0];
	let mut n_2=&s.name[0];
	if coin_toss<5{}else{
		c_1=s_c;
		c_2=a_c;
		n_1=&s.name[0];
		n_2=&a.name[0]
	};	
	
	//PRINT QUESTION.
	let question = format!("A buffer contains {}mol/L {} and {}mol/L {}. \
		{}L of {}mol/L {} is added to {}L of this buffer. \
		What is:\n a) The starting pH?\n b) The final pH? \n c) The pH change?",
		dis(c_1),
		n_1,
		dis(c_2),
		n_2,
		dis(v_strong),
		dis(strong_c),
		strong.name[0],
		dis(v_buf));;
	
	//PRINT ANSWER.
	let mut ans_a=Vec::new();
	ans_a.push(format!("a) Starting pH: Henderson equation!"));
	if a.pka[0].0<7.0{
		ans_a.push(format!("\n   pH = pKa + log([S]/[A])"));
		ans_a.push(format!("\n   pH = pKa + log([{} x {}]/[{} x {}])",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
	}else{
		ans_a.push(format!("\n   pH = pKa + log([B]/[S])"));
		ans_a.push(format!("\n   pH = pKa + log([{} x {}]/[{} x {}])",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));		
	};
	
	ans_a.push(format!("\nb) Final pH:"));
	ans_a.push(format!("\n   {} is a strong {}. After addition of {} the {}.",strong.name[0],marker.0,strong.name[0],marker.1));
	ans_a.push(format!("\n   To find final pH {}.",marker.2));
	ans_a.push(format!("\nc) ΔpH = Final pH -Starting pH.\n"));
	
	ans_a.push(format!("\n{}",format!("a) Starting pH = {}",&ff(4,p_h_start))));
	ans_a.push(format!("\n{}",format!("b) Final pH = {}",&ff(4,p_h_fin))));
	let ans_b=format!("{}",format!("c) ΔpH = {}\n",&ff(4,p_h_fin-p_h_start)));
	let answer = format!("{}\n{}\n",ans_a.join(""),ans_b);
	(question,answer)

}


pub fn q_7_3(compounds:&Vec<Compound>)->(String,String){
//Concs as function of pH (total active ion given).
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	
	//Generate acid or base.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		//let mut acid=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		for y in x.solutes.iter(){
			if y.1=="OH" {gives_oh=true}else{}
		};
		if (((gives_h==true) & (x.pka[0].0<6.0)) || ((gives_oh==true) & (x.pka[0].0>8.0)))
		& (x.salt==false)
		& (x.use_weak==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	
	//is compound A an acid (important!)
	let a_is_acid=if a.pka[0].0<7.0 {true}else{false};
		
	//Generate salt.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut salt_of_a=false;
		for y in x.solutes.iter(){
			if y.1==a.pka[0].1 {salt_of_a=true}else{}
		};
		salt_of_a= if x.salt==false{false}else{salt_of_a};
		if (((a_is_acid==true) & (x.pka[0].0<6.0)) || ((a_is_acid==false) & (x.pka[0].0>8.0)))
		& (x.use_weak==true)
		& (salt_of_a==true){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let s=&valid_c[indx];
	
	//generate concentration of b. (nb salt is usually les soluble so solubility check is here)
	let mut s_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check solubility
	let mut silly=true;
	while silly==true{
		if s.solubility==f64::INFINITY{silly=false; continue
		}else{
			if s_c*s.mmass/10.0>s.solubility{s_c=s_c/10.0}else{silly=false}
		}
	};
	
	//concentration of ions in salt. Kind of.
	let mut nimp_in_s=0.0;
		for y in s.solutes.iter(){
			if y.1==s.pka[0].1 {nimp_in_s=y.0 as f64}else{}
	};
	
	let mut a_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check whether this makes a buffer question.
	let mut silly=true;
	while silly==true{	
		if (a_c*10.0)<s_c*nimp_in_s {s_c=s_c*0.1
		}else if s_c*nimp_in_s<(a_c*0.1) {a_c=a_c*0.1
		}else{silly=false}
	};
	
	//concentration of active ion in A and B
	let mut nimp_in_a=0.0;
	for y in a.solutes.iter(){
			if y.1==a.pka[0].1 {nimp_in_a=y.0 as f64}else{}
	};
	let ion_in_a= a_c*nimp_in_a;
	let ion_in_s= s_c*nimp_in_s;
	
	
	let c_base= if a.pka[0].0<7.0{ion_in_s}else{ion_in_a};
	let	c_acid= if a.pka[0].0<7.0{ion_in_a}else{ion_in_s};
	let c_ion=c_base+c_acid;
	
	let p_h=a.pka[0].0+(c_base/c_acid).log(10.0);
	let coin_toss=rand::thread_rng().gen_range(0,10);
	
	let mut c_1=a_c;
	let mut c_2=s_c;
	let mut n_1=&a.name[0];
	let mut n_2=&s.name[0];
	if coin_toss<5{}else{
		c_1=s_c;
		c_2=a_c;
		n_1=&s.name[0];
		n_2=&a.name[0]
	};	
	
	let species="(".to_owned()+&a.formula[0]+"+"+&a.pka[0].1+")";
	let question = format!("A {} / {} buffer has a total concentration of buffering species equal to {}mol/L and has a pH of {}. What are the concentrations of {} and {}?",
		n_1,
		n_2,
		dis(c_ion),
		ff(4,p_h),
		n_2,
		n_1);
	
	let mut ans_a=Vec::new();
	ans_a.push(format!("Henderson equation!"));
	if a.pka[0].0<7.0{
		ans_a.push(format!("\npH = pKa + log([S]/[A])"));
		ans_a.push(format!("\npH = pKa + log([{} x {}]/[{} x {}])",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		ans_a.push(format!("\n[{}] = {} x [{}] + {} x [{}]",species,nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		if coin_toss<5 {
			ans_a.push(format!("\n[{} x {}] = [{}-({} x {})] x 10^(pH-pKa)",nimp_in_s,s.formula[0],species,nimp_in_a,a.formula[0]));
			ans_a.push(format!("\nSolve this equation for {}.",s.name[0]));
		}else{
			ans_a.push(format!("\n[{} x {}] = [{}-({} x {})] x 10^(pKa-pH)",nimp_in_a,a.formula[0],species,nimp_in_s,s.formula[0]));
			ans_a.push(format!("\nSolve this equation for {}.",&a.name[0]));
		}
	}else{
		ans_a.push(format!("\npH = pKa + log([B]/[S])"));
		ans_a.push(format!("\npH = pKa + log([{} x {}]/[{} x {}])",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));	
		ans_a.push(format!("\n[{}] = {} x [{}] + {} x [{}]",species,nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		if coin_toss<5 {
			ans_a.push(format!("\n[{} x {}] = [{}-({} x {})] x 10^(pH-pKa)",nimp_in_a,a.formula[0],species,nimp_in_s,s.formula[0]));
			ans_a.push(format!("\nSolve this equation for {}.",&a.name[0]));
		}else{
			ans_a.push(format!("\n[{} x {}] = [{}-({} x {})] x 10^(pKa-pH)",nimp_in_s,s.formula[0],species,nimp_in_a,a.formula[0]));
			ans_a.push(format!("\nSolve this equation for {}.",s.name[0]));
		}	
	};
	ans_a.push(format!("\nThen go back and substitute into:\n[{}] = {} x [{}] + {} x [{}]",species,nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));

	ans_a.push(format!("\n{}",format!("\n [{}] = {}mol/L",n_2,dis(c_2))));
	ans_a.push(format!("{}",format!("\n [{}] = {}mol/L\n\n",n_1,dis(c_1))));

	let answer = ans_a.join("");
	(question,answer)
}


pub fn q_7_3b(compounds:&Vec<Compound>)->(String,String){
//Concs as a function of Osmolarity as function of pH (total active ion given).
	//remove medically related compounds
	let mut c_2 = Vec::new();
	for x in compounds.into_iter(){
		if !x.med.0 {c_2.push(x);};
	};
	let compounds = c_2;
	
	//Generate acid or base.
	let mut valid_a:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut gives_h=false;
		//let mut acid=false;
		let mut gives_oh=false;
		for y in x.solutes.iter(){
			if y.1=="H" {gives_h=true}else{}
		};
		for y in x.solutes.iter(){
			if y.1=="OH" {gives_oh=true}else{}
		};
		if (((gives_h==true) & (x.pka[0].0<6.0)) || ((gives_oh==true) & (x.pka[0].0>8.0)))
		& (x.salt==false)
		& (x.use_weak==true){valid_a.push(&x)}else{}
	};
	let va_len=valid_a.len();
	let indx=rand::thread_rng().gen_range(0,va_len);
	let a=&valid_a[indx];
	
	//is compound A an acid (important!)
	let a_is_acid=if a.pka[0].0<7.0 {true}else{false};
		
	//Generate salt.
	let mut valid_c:Vec<&Compound>=Vec::new();
	for x in compounds.iter(){
		let mut salt_of_a=false;
		for y in x.solutes.iter(){
			if y.1==a.pka[0].1 {salt_of_a=true}else{}
		};
		salt_of_a= if x.salt==false{false}else{salt_of_a};
		if (((a_is_acid==true) & (x.pka[0].0<6.0)) || ((a_is_acid==false) & (x.pka[0].0>8.0)))
		& (x.use_weak==true)
		& (salt_of_a==true){valid_c.push(&x)}else{}
	};
	let v_len=valid_c.len();
	let indx=rand::thread_rng().gen_range(0,v_len);
	let s=&valid_c[indx];
	
	//generate concentration of b. (nb salt is usually les soluble so solubility check is here)
	let mut s_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check solubility
	let mut silly=true;
	while silly==true{
		if s.solubility==f64::INFINITY{silly=false; continue
		}else{
			if s_c*s.mmass/10.0>s.solubility{s_c=s_c/10.0}else{silly=false}
		}
	};
	
	//concentration of ions in salt. Kind of.
	let mut nimp_in_s=0.0;
		for y in s.solutes.iter(){
			if y.1==s.pka[0].1 {nimp_in_s=y.0 as f64}else{}
	};
	
	let mut a_c:f64=(rand::thread_rng().gen_range(10,670) as f64)/500.0;
	
	//check whether this makes a buffer question.
	let mut silly=true;
	while silly==true{	
		if (a_c*10.0)<s_c*nimp_in_s {s_c=s_c*0.1
		}else if s_c*nimp_in_s<(a_c*0.1) {a_c=a_c*0.1
		}else{silly=false}
	};
	
	//concentration of active ion in A and B
	let mut nimp_in_a=0.0;
	for y in a.solutes.iter(){
			if y.1==a.pka[0].1 {nimp_in_a=y.0 as f64}else{}
	};
	let ion_in_a= a_c*nimp_in_a;
	let ion_in_s= s_c*nimp_in_s;
	
	
	let c_base= if a.pka[0].0<7.0{ion_in_s}else{ion_in_a};
	let	c_acid= if a.pka[0].0<7.0{ion_in_a}else{ion_in_s};
	
	//calculate osmolarity.
	let mut osma=a_c;
	let mut salt_const=0;
	for x in s.solutes.iter(){
		osma+=s_c*(x.0 as f64);
		salt_const+=x.0};
	
	let p_h=a.pka[0].0+(c_base/c_acid).log(10.0);
	let coin_toss=rand::thread_rng().gen_range(0,10);
	
	let mut c_1=a_c;
	let mut c_2=s_c;
	let mut n_1=&a.name[0];
	let mut n_2=&s.name[0];
	if coin_toss<5{}else{
		c_1=s_c;
		c_2=a_c;
		n_1=&s.name[0];
		n_2=&a.name[0]
	};	
	
	
	let question = format!("A {} / {} buffer has an osmolarity of {}Osmol/L and a pH of {}. What are the concentrations of {} and {}?",
		n_1,
		n_2,
		dis(osma),
		ff(4,p_h),
		n_2,
		n_1);
	
	let mut ans_a=Vec::new();
	ans_a.push(format!("Henderson equation!"));
	if a.pka[0].0<7.0{
		ans_a.push(format!("\npH = pKa + log([S]/[A])"));
		ans_a.push(format!("\npH = pKa + log([{} x {}]/[{} x {}])",nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));
		ans_a.push(format!("\nOsmolarity = Σ(cs)"));
		ans_a.push(format!("\nOsmolarity = "));
		ans_a.push(format!("{} x {}",nimp_in_a,a.formula[0]));
		for x in s.solutes.iter(){ans_a.push(format!(" + {} x {}",x.0,x.1))};
		ans_a.push(format!("\nOsmolarity = {} x [{}] + {} x [{}]",nimp_in_a,a.formula[0],salt_const,s.formula[0]));
		if coin_toss<5 {
			ans_a.push(format!("\n[{} x {}] = [Osmolarity - ({} x {})] x 10^(pH-pKa)",salt_const,s.formula[0],nimp_in_a,a.formula[0]));
			ans_a.push(format!("\nSolve this equation for {}.",s.name[0]));
		}else{
			ans_a.push(format!("\n[{} x {}] = [Osmolarity - ({} x {})] x 10^(pKa-pH)",nimp_in_a,a.formula[0],salt_const,s.formula[0]));
			ans_a.push(format!("\nSolve this equation for {}.",&a.name[0]));
		}
	}else{
		ans_a.push(format!("\npH = pKa + log([B]/[S])"));
		ans_a.push(format!("\npH = pKa + log([{} x {}]/[{} x {}])",nimp_in_a,a.formula[0],nimp_in_s,s.formula[0]));	
		ans_a.push(format!("\nOsmolarity = Σ(cs)"));
		ans_a.push(format!("\nOsmolarity = "));
		ans_a.push(format!("{} x {}",nimp_in_a,a.formula[0]));
		for x in s.solutes.iter(){ans_a.push(format!(" + {} x {}",x.0,x.1))};
		ans_a.push(format!("\nOsmolarity = {} x [{}] + {} x [{}]",nimp_in_a,a.formula[0],salt_const,s.formula[0]));
		if coin_toss<5 {
			ans_a.push(format!("\n[{} x {}] = [Osmolarity-({} x {})] x 10^(pH-pKa)",nimp_in_a,a.formula[0],salt_const,s.formula[0]));
			ans_a.push(format!("\nSolve this equation for {}.",&a.name[0]));
		}else{
			ans_a.push(format!("\n[{} x {}] = [Osmolarity-({} x {})] x 10^(pKa-pH)",salt_const,s.formula[0],nimp_in_a,a.formula[0]));
			ans_a.push(format!("\nSolve this equation for {}.",s.name[0]));
		}	
	};
	ans_a.push(format!("\nThen go back and substitute into:\n[{}]={} x [{}] + {} x [{}]",a.pka[0].1,nimp_in_s,s.formula[0],nimp_in_a,a.formula[0]));

	ans_a.push(format!("\n\n{}",format!(" [{}] = {}mol/L",n_2,dis(c_2))));
	ans_a.push(format!("\n{}",format!(" [{}] = {}mol/L\n",n_1,dis(c_1))));

	let answer = ans_a.join("");
	(question,answer)
}				
//Equilibirum constant questions:

//Will it go left or right.

pub fn q_5_0_pressure(reaction_lib:&Vec<Reaction>)->(String,String) {
	let (mut question,mut answer) = (String::with_capacity(500),String::with_capacity(500));
	
	//Pick 
	let reaction = &reaction_lib[rand::thread_rng().gen_range(0,reaction_lib.len())];
	
	//Write question.
	let increase = if rand::thread_rng().gen_range(0,200)>99 {true}else{false};
	let change = if increase {"increases"}else{"decreases"};
	
	question.push_str("Consider the following reaction:\n\n");
	question.push_str(&reaction.draw_with_state());
	question.push_str(&format!("\n\nIn which direction will the equilibrium shift if the pressure {}?",change));
	
	//work out the answer.
	let mut reagent_nums = 0;
	for x in reaction.reagents.iter() {
		if x.2==GAS {reagent_nums+= x.1;};
	};
	
	let mut product_nums = 0;
	for x in reaction.products.iter() {
		if x.2==GAS {product_nums+= x.1;};
	};
	
	let result =if		(reagent_nums>product_nums) & increase  {"shift to the right"}
				else if (reagent_nums<product_nums) & increase  {"shift to the left"}
				else if (reagent_nums>product_nums) & !increase {"shift to the left"}
				else if (reagent_nums<product_nums) & !increase {"shift to the right"}
				else											{"not change"}
	; 
	
	//Start making the answer.
	answer.push_str(&format!("Moles of reagent in the gas phase: {}\n",reagent_nums));
	answer.push_str(&format!("Moles of product in the gas phase: {}\n",product_nums));
	answer.push_str(&format!("Therefore if pressure {}, equilibrium will {}",change,result));

	(question,answer)
}

//Enthalpy question 

pub fn q_5_0_enthalpy(reactions:&Vec<Reaction>)->(String,String) {
	
	let mut question = String::with_capacity(500);
	let mut answer = String::with_capacity(500);
	let enthalpic_error = ("Nothing to see here.".to_owned(),"Proceed to next question".to_owned());
	
	let mut enth_reactions = Vec::with_capacity(reactions.len());
	
	//Get reactions where enthalpy is used.
	for x in reactions.iter() {
		match x.eq {
			DeltaH(_) => {enth_reactions.push(x);},
			_		  => {},
		};
	};
	
	//Exit if library has no enthalpy based questions.
	if enth_reactions.len()==0 {
		return enthalpic_error
	};
	
	//Pick reaction from valid reaction list.
	let reaction = &enth_reactions[rand::thread_rng().gen_range(0,enth_reactions.len())];
	
	//Write question.
	let increase = if rand::thread_rng().gen_range(0,200)>99 {true}else{false};
	let change = if increase {"increases"}else{"decreases"};
	
	question.push_str("Consider the following reaction:\n\n");
	question.push_str(&reaction.draw_with_hs());
	question.push_str(&format!("\n\nIn which direction will the equilibrium shift if the temperature {}?",change));
	
	let (enth,enth_num) = match reaction.eq {
		DeltaH (x) => {(dis(x),x)},
		_		   => {return enthalpic_error;},
	};
	
	let thermic = if enth_num>0.0 {"endothermic"}else{"exothermic"};
	
	
	let result =if		(enth_num>0.0) & increase  {"shift to the right"}
				else if (enth_num<0.0) & increase  {"shift to the left"}
				else if (enth_num>0.0) & !increase {"shift to the left"}
				else if (enth_num<0.0) & !increase {"shift to the right"}
				else							   {"not change"}
	; 
	
	answer.push_str(&format!("ΔH = {}kJ/mol, therefore the reaction is {}.",enth,thermic));
	answer.push_str(&format!("\nWhen temperature {}, {} reactions will {}.\n",change,thermic,result));
	
	(question,answer)
}

//work out Keq from equilibrium concentrations.
//UNFINISHED.

pub fn q_5_1(reactions:&Vec<Reaction>)->(String,String) {
	
	let mut question = String::with_capacity(500);
	let mut answer = String::with_capacity(500);
	let enthalpic_error = ("Nothing to see here.".to_owned(),"Proceed to next question".to_owned());
	
	let mut keq_reactions = Vec::with_capacity(reactions.len());
	
	
	//Get reactions where keq is used.
	for x in reactions.iter() {
		match x.eq {
			Keq(_) => {keq_reactions.push(x);},
			_		  => {},
		};
	};
	
	//Exit if library has no enthalpy based questions.
	if keq_reactions.len()==0 {
		return enthalpic_error
	};
	
	let reaction:&Reaction = keq_reactions[rand::thread_rng().gen_range(0,keq_reactions.len())];
	
	//Get Keq - NB, this needs to be recalculated at the end.
	let keq:f64 = match reaction.eq {
		Keq(x) => {x*rand::thread_rng().gen_range(0.5,1.5)},
		_	   => {return enthalpic_error;},
	};
	
	//Generate concentration totals between 1mM-5M.
	//NB for whichever is more abundant.
	let sum_concs:f64 = rand::thread_rng().gen_range(1,5001) as f64/5000.0;
	let products = if rand::thread_rng().gen_range(0,200)>99 {true}else{false};
	
	//generate max_conc of a single product.
	let chosen_side = if products {&reaction.products}else{&reaction.reagents};
	let unchosen_side = if products {&reaction.reagents}else{&reaction.products};
	let mut chosen_concs = Vec::new();
	let mut unchosen_concs = Vec::new();
	let mut chosen_side_product = 1.0;
	
	//NB the max_conc isn't actually a limit. More a guideline.
	let max_conc = sum_concs/chosen_side.len() as f64;
	let min = max_conc/1000.0;
	
	//work out product of concentrations of products/reagents.
	for cmp in chosen_side.iter() {
		let conc:f64 = rand::thread_rng().gen_range(min,max_conc);
		chosen_concs.push(conc);
		chosen_side_product*= if cmp.2!=SOL {
			conc.powf(cmp.1 as f64)
		}else{
			1.0
		};
	};
	
	//get the other half of the equation.
	let mut unchosen_side_product = if products {
		chosen_side_product/keq
	}else{
		keq*chosen_side_product
	};
	
	let mut unchosen_num:f64 = unchosen_side.iter().fold(
		0.0,|acc,x|
		if x.2!=SOL {acc + x.1 as f64}else{acc}
	);
	//work out reagent concentrations.
	for i in 0..(unchosen_side.len()-1) {
		if unchosen_side[i].2==SOL {
			let conc:f64 = rand::thread_rng().gen_range(min,max_conc);
			unchosen_concs.push(conc);
		}else{
			let conc:f64 = unchosen_side_product.powf(1.0/(unchosen_side[i].1 as f64*unchosen_num))*rand::thread_rng().gen_range(0.1,10.0);
			unchosen_num-= unchosen_side[i].1 as f64;
			unchosen_side_product/= conc;
			unchosen_concs.push(conc);
		};		
	};
	//Tail case.
	if unchosen_side[unchosen_side.len()-1].2==SOL {
		unchosen_concs.push(rand::thread_rng().gen_range(min,max_conc));
	}else{
		unchosen_concs.push(unchosen_side_product.powf(1.0/unchosen_side[unchosen_side.len()-1].1 as f64));
	};
	
	
	//Correct everything to 4sf.
	for x in unchosen_concs.iter_mut() {*x = ff(4,*x).parse::<f64>().unwrap_or(*x);};
	for x in chosen_concs.iter_mut() {*x = ff(4,*x).parse::<f64>().unwrap_or(*x);};
	
	let mut chosen_side_product = 1.0;
	let mut unchosen_side_product = 1.0;
	
	//work out product of concentrations of products/reagents.
	for (i,_) in chosen_side.iter().enumerate() {
		chosen_side_product*= if chosen_side[i].2!=SOL {
			chosen_concs[i].powf(chosen_side[i].1 as f64)
		}else{
			1.0
		};
	};
	
	//work out product of concentrations of reagents/products.
	for (i,_) in unchosen_side.iter().enumerate() {
		unchosen_side_product*= if unchosen_side[i].2!=SOL {
			unchosen_concs[i].powf(unchosen_side[i].1 as f64)
		}else{
			1.0
		};
	};
	
	//Get new keq based on imaginary values.
	let keq = if products {chosen_side_product/unchosen_side_product}
			  else {unchosen_side_product/chosen_side_product};
	
	//Write question text.
	question.push_str("Consider the following reaction:\n\n");
	question.push_str(&reaction.draw_with_state());
	question.push_str("\n\nWhat is the Keq if the reagent and product concentrations are as follows:\n\n");
	
	
	for i in 0..reaction.reagents.len() {
		question.push_str(
			&format!(
				"[{}] = {}mol/L\n",
				reaction.reagents[i].0,
				if products {dis(unchosen_concs[i])}else{dis(chosen_concs[i])}
			)
		);
	};
	
	for i in 0..reaction.products.len() {
		question.push_str(
			&format!(
				"[{}] = {}mol/L\n",
				reaction.products[i].0,
				if !products {dis(unchosen_concs[i])}else{dis(chosen_concs[i])}
			)
		);
	};
	
	//Write question text.
	answer.push_str(&reaction.draw_eq_equation_activity());
	answer.push_str(&format!("\n\nTherefore Keq = {}\n",ff(4,keq)));
	
	//Reminder if there is a solid reagent.
	let mut solids = false;
	for x in chosen_side.iter() {if x.2==SOL {solids = true;};};
	for x in unchosen_side.iter() {if x.2==SOL {solids = true;};};
	
	if solids {answer.push_str("Remember, solids have an activity of 1.");};
	
	(question,answer)
}
	
	
//Get equilibrium concentration from Keq and initial concs.
//This will get quite complex because we have several cases.
//1) 2 or less products/reagents, no solids.
//2) 2 or more products/reagents, but equal powers above and below (rootable!)
//3) Case 1 + solids. (solids calculated seperately).
//4) Case 2 + solids. (solids calculated seperately).
//The solutions to other cases are too complex to cover in this kind of question.

pub fn q_5_2(reactions:&Vec<Reaction>)->(String,String){
	let (mut q,mut a) = (String::with_capacity(1000),String::with_capacity(1000));
	let enthalpic_error = ("Nothing to see here.".to_owned(),"Proceed to next question".to_owned());
	
	let mut keq_reactions = Vec::with_capacity(reactions.len());
	
	//Get reactions where keq is used.
	for x in reactions.iter() {
		match x.eq {
			Keq(_) => {
				let r_num = x.reagents.iter().fold(0,|ac,r| if r.2!=SOL {ac + r.1}else{ac} );
				let p_num = x.products.iter().fold(0,|ac,p| if p.2!=SOL {ac + p.1}else{ac} );
				if ( (r_num>0) & (p_num>0) ) 
				 & ( (r_num==p_num) | ((r_num<3) & (p_num<3)) )
				{keq_reactions.push(x);};
			},
			_		  => {},
		};
	};
	
	//Exit if library has no enthalpy based questions.
	if keq_reactions.len()==0 {
		return enthalpic_error
	};
	
	let reaction:&Reaction = keq_reactions[rand::thread_rng().gen_range(0,keq_reactions.len())];
	
	//Get Keq - NB, this needs to be recalculated at the end.
	//Also this is unnecessary due to previous safety check, but never mind.
	let keq:f64 = match reaction.eq {
		Keq(x) => {ff(4,x*rand::thread_rng().gen_range(0.5,1.5)).parse().unwrap_or(x)},
		_	   => {return enthalpic_error;},
	};
	
	//get the total powers of reagents and products.
	let r_num = reaction.reagents.iter().fold(0,|ac,r| if r.2!=SOL {ac + r.1}else{ac} );
	let p_num = reaction.products.iter().fold(0,|ac,p| if p.2!=SOL {ac + p.1}else{ac} );
	
	//Generate concentration totals between 1mM-5M.
	//NB for whichever is more abundant.
	let sum_concs:f64 = rand::thread_rng().gen_range(1,5001) as f64/5000.0;
	let products = if rand::thread_rng().gen_range(0,200)>99 {true}else{false};
	
	//generate max_conc of a single product and numbers by side sorted.
	let init_side = if products {&reaction.products}else{&reaction.reagents};
	let zero_side = if products {&reaction.reagents}else{&reaction.products};
	
	let init_num = if products {p_num}else{r_num};
	let zero_num = if products {r_num}else{p_num};
	
	let mut init_concs = Vec::new();
	let mut in_init_concs = Vec::new();
	let mut zero_concs = Vec::new();
	let mut x;
	
	//case more than squared, but same number.
	if (r_num>2) & (p_num==r_num) {
		let keq_root = keq.powf(1.0/r_num as f64);
		let mut conc_init = sum_concs/r_num as f64; //NB this is concentration of abstract component, not compound.
		conc_init = ff(4,conc_init).parse().unwrap_or(conc_init);
		
		x = if !products { keq_root*conc_init }
				else	 { conc_init/keq_root };
				
		for _ in 0..init_side.len() {init_concs.push(conc_init);};
		for _ in 0..zero_side.len() {zero_concs.push(x);};		
		
	//case of squared or less.
	}else if (r_num<3) & (p_num<3) {
		//work out product of concentrations of initial side.
		//and *final* concentrations. (Easier this way).
		let mut init_side_product = 1.0;
		let max_conc = sum_concs/init_side.len() as f64;
		let min = max_conc/1000.0;
		
		for (i,_) in init_side.iter().enumerate() {
			let mut conc:f64 = rand::thread_rng().gen_range(min,max_conc);
			conc = ff(4,conc).parse().unwrap_or(conc);
			
			init_concs.push(conc);
			init_side_product*= if init_side[i].2!=SOL {
				conc.powf(init_side[i].1 as f64)
			}else{
				1.0
			};
		};
		
		//determine final product of zero side.
		let zero_side_product = if !products {
			keq*init_side_product
		}else{
			init_side_product/keq
		};
		
		//determine x.
		x = zero_side_product.powf(1.0/zero_num as f64);
		
		//get final concentrations.
		for _ in zero_side.iter() {zero_concs.push(x);};
	}else{
		return enthalpic_error;
	};
	
	//correct initial concentrations to true initial concentrations.
	for conc in init_concs.iter() {in_init_concs.push(*conc+x);};	
	
	//Initial concs are all exact. Thus only final concs need reworked.
	for x in zero_concs.iter_mut() {*x = ff(4,*x).parse().unwrap_or(*x);};
	
	//work out products of concentrations (as prelude to recalculating the keq.
	let zero_prod = zero_concs.iter().zip(zero_side.iter()).fold(
		1.0,|ac,(conc,cmp)|
		if cmp.2!=SOL {ac*conc.powf(cmp.1 as f64)}else{ac}
	);
	
	x = zero_prod.powf(1.0/zero_num as f64);
	
	//Write question text.
	q.push_str("Consider the following reaction:\n\n");
	q.push_str(&reaction.draw_with_state());
	q.push_str(&format!("\n\nWhat the are the equilibrium \
concentrations of reagents and products if \
Keq = {} and initial concentrations are as follows:\n\n",keq));
	
	for i in 0..reaction.reagents.len() {
		q.push_str(
			&format!(
				"[{}] = {}mol/L\n",
				reaction.reagents[i].0,
				if products {"0.0 ".to_owned()}else{dis(in_init_concs[i])}
			)
		);
	};
	
	for i in 0..reaction.products.len() {
		q.push_str(
			&format!(
				"[{}] = {}mol/L\n",
				reaction.products[i].0,
				if !products {"0.0 ".to_owned()}else{dis(in_init_concs[i])}
			)
		);
	};
	
	//write answer.
	a.push_str("The equilibrium equation for this reaction looks like this:\n\n");
	a.push_str(&reaction.draw_eq_equation_activity());
	a.push_str("\n\nEquilibrium concentrations would thus look like this:\n\n");
	a.push_str("Keq = ");
	
	if !products {
		a.push_str(&format!("x^({})",zero_num));
		a.push_str(" / ");
		//A little complication because of powers and roots.
		if (r_num>2) & (p_num==r_num) {
			a.push_str(&format!("({}-x)^({})",ff(4,init_concs[0]),init_num));
		}else{
			for (c,cmp) in in_init_concs.iter().zip(init_side.iter()) {
				if cmp.2!=SOL {a.push_str(&format!("({}-x)^({})",ff(4,*c),cmp.1))};
			};
		};
	}else{
		//A little complication because of powers and roots.
		if (r_num>2) & (p_num==r_num) {
			a.push_str(&format!("({}-x)^({})",ff(4,init_concs[0]),init_num));
		}else{
			for (c,cmp) in in_init_concs.iter().zip(init_side.iter()) {
				if cmp.2!=SOL {a.push_str(&format!("({}-x)^({})",ff(4,*c),cmp.1))};
			};
		};
		a.push_str(" / ");
		a.push_str(&format!("x^({})",zero_num));
	};
	a.push('\n');
	if (r_num>2) & (p_num==r_num) {a.push_str(&format!("\nPut both sides of the equation to the power of 1/{} and... ",r_num));};
	a.push_str("\nSolve for x. Use x to work out individual concentrations. Thus:\n\n");
	a.push_str(&format!("x = {}\n",x));
	
	for i in 0..reaction.reagents.len() {
		a.push_str(
			&format!(
				"[{}] = {}mol/L\n",
				reaction.reagents[i].0,
				if products {dis(zero_concs[i])}else{dis(init_concs[i])}
			)
		);
	};
	
	for i in 0..reaction.products.len() {
		a.push_str(
			&format!(
				"[{}] = {}mol/L\n",
				reaction.products[i].0,
				if !products {dis(zero_concs[i])}else{dis(init_concs[i])}
			)
		);
	};
	
	//NB: Need to write answer.
	(q,a)
}	

//HOUSEKEEPING FUNCTIONS. BOILERPLATE FORMATTING. KEEP OUT.
//HOUSEKEEPING FUNCTIONS. BOILERPLATE FORMATTING. KEEP OUT.
//HOUSEKEEPING FUNCTIONS. BOILERPLATE FORMATTING. KEEP OUT.
//HOUSEKEEPING FUNCTIONS. BOILERPLATE FORMATTING. KEEP OUT.
//HOUSEKEEPING FUNCTIONS. BOILERPLATE FORMATTING. KEEP OUT.
//HOUSEKEEPING FUNCTIONS. BOILERPLATE FORMATTING. KEEP OUT.
//HOUSEKEEPING FUNCTIONS. BOILERPLATE FORMATTING. KEEP OUT.
//HOUSEKEEPING FUNCTIONS. BOILERPLATE FORMATTING. KEEP OUT.
//HOUSEKEEPING FUNCTIONS. BOILERPLATE FORMATTING. KEEP OUT.


pub fn create_reaction_lib()->Vec<Reaction> {
	vec![
		//Reactions with Enthalpy.
		Reaction { //combustion of methane.
			reagents: vec![("O\u{2082}".to_owned(),2,GAS),
						   ("CH\u{2084}".to_owned(),1,GAS)
						  ],
			products: vec![("CO\u{2082}".to_owned(),1,GAS),
						   ("H\u{2082}O".to_owned(),2,GAS)
						  ],
			eq: DeltaH(-891.0),
		},
		Reaction { //oxidation of methane to CO.
			reagents: vec![("H\u{2082}O".to_owned(),1,GAS),
						   ("CH\u{2084}".to_owned(),1,GAS)
						  ],
			products: vec![("CO".to_owned(),1,GAS),
						   ("H\u{2082}".to_owned(),3,GAS)
						  ],
			eq: DeltaH(210.0),
		},
		Reaction { //oxidation sulphur dioxide
			reagents: vec![("SO\u{2082}".to_owned(),2,GAS),
						   ("O\u{2082}".to_owned(),1,GAS)
						  ],
			products: vec![
						   ("SO\u{2083}".to_owned(),2,GAS)
						  ],
			eq: DeltaH(-98.0),
		},
		Reaction { //combustion of hydrogen
			reagents: vec![("H\u{2082}".to_owned(),2,GAS),
						   ("O\u{2082}".to_owned(),1,GAS)
						  ],
			products: vec![
						   ("H\u{2082}O".to_owned(),2,LIQ)
						  ],
			eq: DeltaH(-572.0),
		},
		Reaction { //oxidation of ammonia
			reagents: vec![("NH\u{2083}".to_owned(),4,GAS),
						   ("O\u{2082}".to_owned(),5,GAS)
						  ],
			products: vec![
						   ("NO".to_owned(),4,GAS),
						   ("H\u{2082}O".to_owned(),6,LIQ)
						  ],
			eq: DeltaH(-908.0),
		},
		//
		//Reactions with Equilibrium Constant.
		Reaction { //Succinate to fumerate oxidation.
			reagents: vec![("Succinate".to_owned(),1,AQU),
						   ("FAD".to_owned(),1,AQU)
						  ],
			products: vec![
						   ("Fumarate".to_owned(),1,AQU),
						   ("FADH\u{2082}".to_owned(),1,AQU)
						  ],
			eq: Keq(1.5),
		},
		Reaction { //Chlorination of ethane.
			reagents: vec![("C\u{2082}H\u{2086}".to_owned(),1,GAS),
						   ("Cl\u{2082}".to_owned(),1,GAS)
						  ],
			products: vec![
						   ("C\u{2082}H\u{2085}Cl".to_owned(),1,SOL),
						   ("HCl".to_owned(),1,GAS)
						  ],
			eq: Keq(0.1),
		},
		Reaction { //Condensation to Sucrose.
			reagents: vec![("UDP-Glucose".to_owned(),1,AQU),
						   ("Fructose".to_owned(),1,AQU)
						  ],
			products: vec![
						   ("UDP".to_owned(),1,AQU),
						   ("Sucrose".to_owned(),1,AQU)
						  ],
			eq: Keq(1.6),
		},
		Reaction { //Creatine phosphorylation (backwards).
			reagents: vec![("Creatine".to_owned(),1,AQU),
						   ("ATP".to_owned(),1,AQU)
						  ],
			products: vec![
						   ("Creatine-Pi".to_owned(),1,AQU),
						   ("ADP".to_owned(),1,AQU)
						  ],
			eq: Keq(0.04),
		},
		Reaction { //Isomerisation of glucose.
			reagents: vec![("UDP-Glucose".to_owned(),1,AQU)
						  ],
			products: vec![
						   ("UDP-Galactose".to_owned(),1,AQU)
						  ],
			eq: Keq(0.33),
		},
		Reaction {//Glycolysis step
			reagents: vec![("G3P".to_owned(),1,AQU),
						  ],
			products: vec![
						   ("DHAP".to_owned(),1,AQU)
						  ],
			eq: Keq(22.0),
		},
		Reaction {//Citric acid cycle step
			reagents: vec![("C\u{2084}H\u{2084}O\u{2084}".to_owned(),1,AQU),
						   ("H\u{2082}O".to_owned(),2,LIQ)
						  ],
			products: vec![
						   ("C\u{2084}H\u{2086}O\u{2085}".to_owned(),1,AQU)
						  ],
			eq: Keq(4.03),
		},
		Reaction { //Complex test reaction.
			reagents: vec![("A".to_owned(),1,SOL),
						   ("B".to_owned(),2,AQU),
						   ("C".to_owned(),3,AQU)
						  ],
			products: vec![("E".to_owned(),5,SOL),
						   ("D".to_owned(),2,AQU),
						   ("F".to_owned(),3,GAS)
						  ],
			eq: Keq(1.5),
		},
	]
}
#[allow(unused_assignments)]
pub fn create_compound_lib(mut a:Vec<Compound>)->Vec<Compound>{
	
	a = vec![
		Compound{
			name:vec!["Lithium Chloride".to_owned()],
			formula:vec!["LiCl".to_owned()],
			mmass:42.39,
			solutes:vec![(1,"Li".to_owned(),1),(1,"Cl".to_owned(),-1)],
			solubility:74.48,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Sodium Chloride".to_owned()],
			formula:vec!["NaCl".to_owned()],
			mmass:58.44,
			solutes:vec![(1,"Na".to_owned(),1),(1,"Cl".to_owned(),-1)],
			solubility:35.90,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Potassium Chloride".to_owned()],
			formula:vec!["KCl".to_owned()],
			mmass:74.55,
			solutes:vec![(1,"K".to_owned(),1),(1,"Cl".to_owned(),-1)],
			solubility:34.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Lithium Fluoride".to_owned()],
			formula:vec!["LiF".to_owned()],
			mmass:25.94,
			solutes:vec![(1,"Li".to_owned(),1),(1,"F".to_owned(),-1)],
			solubility:0.127,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Sodium Fluoride".to_owned()],
			formula:vec!["NaF".to_owned()],
			mmass:41.99,
			solutes:vec![(1,"Na".to_owned(),1),(1,"F".to_owned(),-1)],
			solubility:4.04,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Potassium Fluoride".to_owned()],
			formula:vec!["KF".to_owned()],
			mmass:58.10,
			solutes:vec![(1,"K".to_owned(),1),(1,"F".to_owned(),-1)],
			solubility:373.6,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Magnesium Chloride".to_owned()],
			formula:vec!["MgCl\u{2082}".to_owned(),"MgCl2".to_owned()],
			mmass:95.21,
			solutes:vec![(1,"Mg".to_owned(),2),(2,"Cl".to_owned(),-1)],
			solubility:54.30,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Calcium Chloride".to_owned()],
			formula:vec!["CaCl\u{2082}".to_owned(),"CaCl2".to_owned()],
			mmass:111.0,
			solutes:vec![(1,"Ca".to_owned(),2),(2,"Cl".to_owned(),-1)],
			solubility:74.50,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Magnesium Bromide".to_owned()],
			formula:vec!["MgBr\u{2082}".to_owned(),"MgBr2".to_owned()],
			mmass:184.1,
			solutes:vec![(1,"Mg".to_owned(),2),(2,"Br".to_owned(),-1)],
			solubility:102.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Calcium Bromide".to_owned()],
			formula:vec!["CaBr\u{2082}".to_owned(),"CaBr2".to_owned()],
			mmass:199.9,
			solutes:vec![(1,"Ca".to_owned(),2),(2,"Br".to_owned(),-1)],
			solubility:143.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Magnesium Iodide".to_owned()],
			formula:vec!["MgI\u{2082}".to_owned(),"MgI2".to_owned()],
			mmass:278.1,
			solutes:vec![(1,"Mg".to_owned(),2),(2,"I".to_owned(),-1)],
			solubility:148.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Calcium Iodide".to_owned()],
			formula:vec!["CaI\u{2082}".to_owned(),"CaI2".to_owned()],
			mmass:293.9,
			solutes:vec![(1,"Ca".to_owned(),2),(2,"I".to_owned(),-1)],
			solubility:66.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Iron(II) Chloride".to_owned(),"Iron Chloride".to_owned()],
			formula:vec!["FeCl\u{2082}".to_owned(),"FeCl2".to_owned()],
			mmass:126.8,
			solutes:vec![(1,"Fe".to_owned(),2),(2,"Cl".to_owned(),-1)],
			solubility:68.5,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Iron(III) Chloride".to_owned(),"Iron Chloride".to_owned()],
			formula:vec!["FeCl\u{2083}".to_owned(),"FeCl3".to_owned()],
			mmass:162.2,
			solutes:vec![(1,"Fe".to_owned(),3),(3,"Cl".to_owned(),-1)],
			solubility:91.20,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Copper(II) Chloride".to_owned(),"Copper Chloride".to_owned()],
			formula:vec!["CuCl\u{2082}".to_owned(),"CuCl2".to_owned()],
			mmass:134.5,
			solutes:vec![(1,"Cu".to_owned(),2),(2,"Cl".to_owned(),-1)],
			solubility:75.7,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Copper(I) Chloride".to_owned(),"Copper Chloride".to_owned()],
			formula:vec!["CuCl".to_owned()],
			mmass:99.00,
			solutes:vec![(1,"Cu".to_owned(),1),(1,"Cl".to_owned(),-1)],
			solubility:0.047,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Zinc Chloride".to_owned(),"Zinc(II) Chloride".to_owned()],
			formula:vec!["ZnCl\u{2082}".to_owned(),"ZnCl2".to_owned()],
			mmass:134.5,
			solutes:vec![(1,"Zn".to_owned(),2),(2,"Cl".to_owned(),-1)],
			solubility:395.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Aluminium Chloride".to_owned()],
			formula:vec!["AlCl\u{2083}".to_owned(),"AlCl3".to_owned()],
			mmass:133.3,
			solutes:vec![(1,"Al".to_owned(),3),(3,"Cl".to_owned(),-1)],
			solubility: 45.8,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Potassium Sulphate".to_owned()],
			formula:vec!["K\u{2082}SO\u{2084}".to_owned(),"K2SO4".to_owned()],
			mmass:174.3,
			solutes:vec![(2,"K".to_owned(),1),(1,"SO\u{2084}".to_owned(),-2)],
			solubility: 11.1,
			pka:vec![(7.0,"".to_owned())], //Not strictly true.
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Magnesium Sulphate".to_owned()],
			formula:vec!["MgSO\u{2084}".to_owned(),"MgSO4".to_owned()],
			mmass:120.4,
			solutes:vec![(1,"Mg".to_owned(),2),(1,"SO\u{2084}".to_owned(),-2)],
			solubility: 35.1,
			pka:vec![(7.0,"".to_owned())], //Not strictly true.
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Calcium Sulphate".to_owned()],
			formula:vec!["CaSO\u{2084}".to_owned(),"CaSO4".to_owned()],
			mmass:136.1,
			solutes:vec![(1,"Ca".to_owned(),2),(1,"SO\u{2084}".to_owned(),-2)],
			solubility: 0.210,
			pka:vec![(7.0,"".to_owned())], //Not strictly true.
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Copper(II) Sulphate".to_owned(),"Copper Sulphate".to_owned()],
			formula:vec!["CuSO\u{2084}".to_owned(),"CuSO4".to_owned()],
			mmass:159.6,
			solutes:vec![(1,"Cu".to_owned(),2),(1,"SO\u{2084}".to_owned(),-2)],
			solubility: 20.3,
			pka:vec![(7.0,"".to_owned())], //Not strictly true.
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Aluminium Sulphate".to_owned()],
			formula:vec!["Al\u{2082}(SO\u{2084})\u{2083}".to_owned(),"Al2(SO4)3".to_owned()],
			mmass:342.2,
			solutes:vec![(2,"Al".to_owned(),3),(3,"SO\u{2084}".to_owned(),-2)],
			solubility: 36.4,
			pka:vec![(7.0,"".to_owned())], //Not strictly true.
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Trisodium Phosphate".to_owned()],
			formula:vec!["Na\u{2083}PO\u{2084}".to_owned(),"Na3PO4".to_owned()],
			mmass:163.9,
			solutes:vec![(3,"Na".to_owned(),1),(1,"PO\u{2084}".to_owned(),-3)],
			solubility: 12.0,
			pka:vec![(7.0,"".to_owned())], //Not strictly true.
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Iron(III) Phosphate".to_owned(),"Iron Phosphate".to_owned()],
			formula:vec!["FePO\u{2084}".to_owned(),"FePO4".to_owned()],
			mmass:150.8,
			solutes:vec![(1,"Fe".to_owned(),3),(1,"PO\u{2084}".to_owned(),-3)],
			solubility: 0.642,
			pka:vec![(7.0,"".to_owned())], //Not strictly true.
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Magnesium Phosphate".to_owned()],
			formula:vec!["Mg\u{2083}(PO\u{2084})\u{2082}".to_owned(),"Mg3(PO4)2".to_owned()],
			mmass:262.9,
			solutes:vec![(3,"Mg".to_owned(),2),(2,"PO\u{2084}".to_owned(),-3)],
			solubility:2.59*TEN.powi(-4),
			pka:vec![(7.0,"".to_owned())], //Not strictly true.
			use_weak:false,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 
		 Compound{
			name:vec!["Sodium Hydroxide".to_owned()],
			formula:vec!["NaOH".to_owned()],
			mmass:40.0,
			solutes:vec![(1,"Na".to_owned(),1),(1,"OH".to_owned(),-1)],
			solubility: 111.0,
			pka:vec![(14.93,"Na".to_owned())],
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 
		 Compound{
			name:vec!["Potassium Hydroxide".to_owned()],
			formula:vec!["KOH".to_owned()],
			mmass:56.11,
			solutes:vec![(1,"K".to_owned(),1),(1,"OH".to_owned(),-1)],
			solubility: 121.0,
			pka:vec![(14.93,"K".to_owned())],
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Magnesium Hydroxide".to_owned()],
			formula:vec!["Mg(OH)\u{2082}".to_owned(),"Mg(OH)2".to_owned()],
			mmass:58.32,
			solutes:vec![(1,"Mg".to_owned(),2),(2,"OH".to_owned(),-1)],
			solubility:0.00064,
			pka:vec![(14.0,"Mg".to_owned())], //Not strictly true.
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Calcium Hydroxide".to_owned()],
			formula:vec!["Ca(OH)\u{2082}".to_owned(),"Ca(OH)2".to_owned()],
			mmass:74.09,
			solutes:vec![(1,"Ca".to_owned(),2),(2,"OH".to_owned(),-1)],
			solubility:0.173,
			pka:vec![(12.63,"Ca".to_owned())], //NB (12.63,11.57)
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Hydrochloric acid".to_owned()],
			formula:vec!["HCl".to_owned()],
			mmass:36.46,
			solutes:vec![(1,"H".to_owned(),1),(1,"Cl".to_owned(),-1)],
			solubility: f64::INFINITY,
			pka:vec![(-6.3,"Cl".to_owned())],
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Sulphuric Acid".to_owned()],
			formula:vec!["H\u{2082}SO\u{2084}".to_owned(),"H2SO4".to_owned()],
			mmass:98.08,
			solutes:vec![(2,"H".to_owned(),1),(1,"SO\u{2084}".to_owned(),-2)],
			solubility: f64::INFINITY,
			pka:vec![(-3.0,"SO\u{2084}".to_owned())], //Not strictly true.(-3.0 and 2)
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Pyruvic Acid".to_owned()],
			formula:vec!["CH\u{2083}COCOOH".to_owned(),"CH3COCOOH".to_owned()],
			mmass:88.06,
			solutes:vec![(1,"H".to_owned(),1),(1,"CH\u{2083}COCOO".to_owned(),-1)],
			solubility: 100.0,
			pka:vec![(2.5,"CH\u{2083}COCOO".to_owned())],
			use_weak:true,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Sodium Pyruvate".to_owned()],
			formula:vec!["NaCH\u{2083}COCOO".to_owned(),"NaCH3COCOO".to_owned()],
			mmass:110.04,
			solutes:vec![(1,"Na".to_owned(),1),(1,"CH\u{2083}COCOO".to_owned(),-1)],
			solubility: 10.0,
			pka:vec![(2.5,"CH\u{2083}COCOO".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Formic Acid".to_owned(),"Methanoic Acid".to_owned()],
			formula:vec!["HCOOH".to_owned(),"CHOOH".to_owned()],
			mmass:46.03,
			solutes:vec![(1,"H".to_owned(),1),(1,"HCOO".to_owned(),-1)],
			solubility: f64::INFINITY,
			pka:vec![(3.77,"HCOO".to_owned())],
			use_weak:true,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Potassium Formate".to_owned(),"Potassium Methanoate".to_owned()],
			formula:vec!["KHCOO".to_owned(),"KCHOO".to_owned()],
			mmass:84.12,
			solutes:vec![(1,"K".to_owned(),1),(1,"HCOO".to_owned(),-1)],
			solubility: 335.0,
			pka:vec![(3.77,"HCOO".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Calcium Formate".to_owned(),"Calcium Methanoate".to_owned()],
			formula:vec!["Ca(HCOO)\u{2082}".to_owned(),"Ca(HCOO)2".to_owned(),"Ca(CHOO)2".to_owned()],
			mmass:130.1,
			solutes:vec![(1,"Ca".to_owned(),2),(2,"HCOO".to_owned(),-1)],
			solubility: 16.6,
			pka:vec![(3.77,"HCOO".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Acetic Acid".to_owned(),"Ethanoic Acid".to_owned()],
			formula:vec!["CH\u{2083}COOH".to_owned(),"CH3COOH".to_owned()],
			mmass:60.05,
			solutes:vec![(1,"H".to_owned(),1),(1,"CH\u{2083}COO".to_owned(),-1)],
			solubility: f64::INFINITY,
			pka:vec![(4.75,"CH\u{2083}COO".to_owned())],
			use_weak:true,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Sodium Acetate".to_owned(),"Sodium Ethanoate".to_owned()],
			formula:vec!["NaCH\u{2083}COO".to_owned(),"NaCH3COO".to_owned()],
			mmass:82.03,
			solutes:vec![(1,"Na".to_owned(),1),(1,"CH\u{2083}COO".to_owned(),-1)],
			solubility: 123.3,
			pka:vec![(4.75,"CH\u{2083}COO".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Magnesium Acetate".to_owned(),"Magnesium Ethanoate".to_owned()],
			formula:vec!["Mg(CH\u{2083}COO)\u{2082}".to_owned(),"Mg(CH3COO)2".to_owned()],
			mmass:142.4,
			solutes:vec![(1,"Mg".to_owned(),2),(2,"CH\u{2083}COO".to_owned(),-1)],
			solubility: 65.6,
			pka:vec![(4.75,"CH\u{2083}COO".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Aluminium Acetate".to_owned(),"Aluminium Ethanoate".to_owned()],
			formula:vec!["Al(CH\u{2083}COO)\u{2083}".to_owned(),"Al(CH3COO)3".to_owned()],
			mmass:204.1,
			solutes:vec![(1,"Al".to_owned(),3),(3,"CH\u{2083}COO".to_owned(),-1)],
			solubility: 14.8,
			pka:vec![(4.75,"CH\u{2083}COO".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		},
		Compound{
			name:vec!["Salicylic Acid".to_owned()],
			formula:vec!["C\u{2087}H\u{2086}O\u{2083}".to_owned(),"C7H6O3".to_owned()],
			mmass:138.1,
			solutes:vec![(1,"H".to_owned(),1),(1,"C\u{2087}H\u{2085}O\u{2083}".to_owned(),-1)],
			solubility: 0.248,
			pka:vec![(2.97,"C\u{2087}H\u{2085}O\u{2083}".to_owned())],
			use_weak:true,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		Compound{
			name:vec!["Sodium Salicylate".to_owned()],
			formula:vec!["NaC\u{2087}H\u{2085}O\u{2083}".to_owned(),"NaC7H5O3".to_owned()],
			mmass:160.1,
			solutes:vec![(1,"Na".to_owned(),1),(1,"C\u{2087}H\u{2085}O\u{2083}".to_owned(),-1)],
			solubility: 124.6,
			pka:vec![(2.97,"C\u{2087}H\u{2085}O\u{2083}".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		Compound{
			name:vec!["Magnesium Salicylate".to_owned()],
			formula:vec!["Mg(C\u{2087}H\u{2085}O\u{2083})\u{2082}".to_owned(),"Mg(C7H5O3)2".to_owned()],
			mmass:298.5,
			solutes:vec![(1,"Mg".to_owned(),2),(2,"C\u{2087}H\u{2085}O\u{2083}".to_owned(),-1)],
			solubility: 0.00686, 
			pka:vec![(2.97,"C\u{2087}H\u{2085}O\u{2083}".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Ammonia".to_owned()],
			formula:vec!["NH\u{2083}".to_owned(),"NH3".to_owned()],
			mmass:17.03,
			solutes:vec![(1,"NH\u{2084}".to_owned(),1),(1,"OH".to_owned(),-1)],
			solubility: f64::INFINITY,
			pka:vec![(9.25,"NH\u{2084}".to_owned())],
			use_weak:true,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Diethylammonium Chloride".to_owned()],
			formula:vec!["(CH\u{2083}CH\u{2082})\u{2082}NH\u{2082}Cl".to_owned(),"(CH3CH2)2NH2Cl".to_owned()],
			mmass:109.6,
			solutes:vec![(1,"(CH\u{2083}CH\u{2082})\u{2082}NH\u{2082}".to_owned(),1),(1,"Cl".to_owned(),-1)],
			solubility: 51.0,
			pka:vec![(10.8,"(CH\u{2083}CH\u{2082})\u{2082}NH\u{2082}".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Diethylamine".to_owned()],
			formula:vec!["(CH\u{2083}CH\u{2082})\u{2082}NH".to_owned(),"(CH3CH2)2NH".to_owned()],
			mmass:73.14,
			solutes:vec![(1,"(CH\u{2083}CH\u{2082})\u{2082}NH\u{2082}".to_owned(),1),(1,"OH".to_owned(),-1)],
			solubility: f64::INFINITY,
			pka:vec![(10.8,"(CH\u{2083}CH\u{2082})\u{2082}NH\u{2082}".to_owned())],
			use_weak:true,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Ammonium Chloride".to_owned()],
			formula:vec!["NH\u{2084}Cl".to_owned(),"NH4Cl".to_owned()],
			mmass:53.49,
			solutes:vec![(1,"NH\u{2084}".to_owned(),1),(1,"Cl".to_owned(),-1)],
			solubility: 39.5,
			pka:vec![(9.25,"NH\u{2084}".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Ammonium Sulphate".to_owned()],
			formula:vec!["(NH\u{2084})\u{2082}SO\u{2084}".to_owned(),"(NH4)2SO4".to_owned()],
			mmass:132.1,
			solutes:vec![(2,"NH\u{2084}".to_owned(),1),(1,"SO\u{2084}".to_owned(),-2)],
			solubility: 70.6,
			pka:vec![(9.25,"NH\u{2084}".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Ammonium Phosphate".to_owned()],
			formula:vec!["(NH\u{2084})\u{2083}HPO\u{2084}".to_owned(),"(NH4)3HPO4".to_owned()],
			mmass:149.0,
			solutes:vec![(3,"NH\u{2084}".to_owned(),1),(1,"PO\u{2084}".to_owned(),-3)],
			solubility: 58.0,
			pka:vec![(9.25,"NH\u{2084}".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Glucose".to_owned()],
			formula:vec!["C\u{2086}H\u{2081}\u{2082}O\u{2086}".to_owned(),"C6H12O6".to_owned()],
			mmass:180.2,
			solutes:vec![(1,"C\u{2086}H\u{2081}\u{2082}O\u{2086}".to_owned(),0)],
			solubility: 90.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Fructose".to_owned()],
			formula:vec!["C\u{2086}H\u{2081}\u{2082}O\u{2086}".to_owned(),"C6H12O6".to_owned()],
			mmass:180.2,
			solutes:vec![(1,"C\u{2086}H\u{2081}\u{2082}O\u{2086}".to_owned(),0)],
			solubility: 375.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Sucrose".to_owned()],
			formula:vec!["C\u{2081}\u{2081}H\u{2082}\u{2082}O\u{2081}\u{2081}".to_owned(),"C11H22O11".to_owned()],
			mmass:342.3,
			solutes:vec![(1,"C\u{2081}\u{2081}H\u{2082}\u{2082}O\u{2081}\u{2081}".to_owned(),0)],
			solubility: 200.0,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:false,
			med: (false,0.0,0.0,"".to_owned(),None),
		 },
		 Compound{
			name:vec!["Ampicillin".to_owned()],
			formula:vec!["C\u{2081}\u{2086}H\u{2081}\u{2089}N\u{2083}O\u{2084}S".to_owned(),"C16H19N3O4S".to_owned()],
			mmass:349.4,
			solutes:vec![(1,"C\u{2081}\u{2086}H\u{2081}\u{2089}N\u{2083}O\u{2084}S".to_owned(),0)],
			solubility: 1.01,
			pka:vec![(2.5,"".to_owned())],
			use_weak:false,
			salt:false,
			med: (true,0.5,2.0,"g".to_owned(),None),
		 },
		 Compound{
			name:vec!["Insulin".to_owned()],
			formula:vec!["Insulin".to_owned(),"Insulin".to_owned()],
			mmass:5808.0,
			solutes:vec![(1,"Insulin".to_owned(),0)],
			solubility: f64::NAN,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:false,
			med: (true,25.0,100.0,"U".to_owned(),Some(0.04034)),  //CFnversion factor in g/U
		 },
		 Compound{
			name:vec!["Ketamine".to_owned()],
			formula:vec!["C\u{2081}\u{2083}H\u{2081}\u{2086}ClNO".to_owned(),"C13H16ClNO".to_owned()],
			mmass:237.7,
			solutes:vec![(1,"C\u{2081}\u{2083}H\u{2081}\u{2086}ClNO".to_owned(),0)],
			solubility: 0.28,
			pka:vec![(7.5,"".to_owned())],
			use_weak:false,
			salt:false,
			med: (true,0.05,0.5,"".to_owned(),None),  //CF
		 },
		 Compound{
			name:vec!["Dexamethasone".to_owned()],
			formula:vec!["C\u{2082}\u{2082}H\u{2082}\u{2089}FO\u{2085}".to_owned(),"C22H29FO5".to_owned()],
			mmass:392.5,
			solutes:vec![(1,"C\u{2082}\u{2082}H\u{2082}\u{2089}FO\u{2085}".to_owned(),0)],
			solubility: 0.0089,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:false,
			med: (true,0.01,0.03,"".to_owned(),None),  //CF in g/mol
		 },
		 Compound{
			name:vec!["Doxycycline".to_owned()],
			formula:vec!["C\u{2082}\u{2082}H\u{2082}\u{2084}N\u{2082}O\u{2088}".to_owned(),"C22H24N2O8".to_owned()],
			mmass:444.4,
			solutes:vec![(1,"C\u{2082}\u{2082}H\u{2082}\u{2084}N\u{2082}O\u{2088}".to_owned(),0)],
			solubility: 0.063,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:false,
			med: (true,0.1,0.2,"".to_owned(),None),  //CF in g/mol
		 },
		 Compound{
			name:vec!["Nystatin".to_owned()],
			formula:vec!["C\u{2084}\u{2087}H\u{2087}\u{2085}NO\u{2081}\u{2087}".to_owned(),"C47H75NO17".to_owned()],
			mmass:926.1,
			solutes:vec![(1,"C\u{2084}\u{2087}H\u{2087}\u{2085}NO\u{2081}\u{2087}".to_owned(),0)],
			solubility: 0.036,
			pka:vec![(7.0,"".to_owned())],
			use_weak:false,
			salt:false,
			med: (true,100000.0,1000000.0,"".to_owned(),Some(0.0000002273)),  //CF in g/U
		 },
		 Compound{
			name:vec!["Acyclovir".to_owned()],
			formula:vec!["C\u{2088}H\u{2081}\u{2081}N\u{2085}O\u{2083}".to_owned(),"C8H11N5O3".to_owned()],
			mmass:225.2,
			solutes:vec![(1,"C\u{2088}H\u{2081}\u{2081}N\u{2085}O\u{2083}".to_owned(),0)],
			solubility: 0.162,
			pka:vec![(2.27,"".to_owned()),(9.25,"".to_owned())],  //and 9.25
			use_weak:true,
			salt:false,
			med: (true,0.2,0.4,"".to_owned(),None),  //CF
		 },
		 Compound{
			name:vec!["Chloroquine".to_owned()],
			formula:vec!["C\u{2081}\u{2088}H\u{2082}\u{2086}ClN\u{2083}".to_owned(),"C18H26ClN3".to_owned()],
			mmass:319.9,
			solutes:vec![(1,"C\u{2081}\u{2088}H\u{2082}\u{2086}ClN\u{2083}".to_owned(),0)],
			solubility: 0.00106,
			pka:vec![(10.1,"".to_owned())], 
			use_weak:true,
			salt:false,
			med: (false,0.5,1.0,"".to_owned(),None),  //CF
		 },
		 Compound{
			name:vec!["Doxorubicin".to_owned()],
			formula:vec!["C\u{2082}\u{2087}H\u{2082}\u{2089}NO\u{2081}\u{2081}".to_owned(),"C27H29NO11".to_owned()],
			mmass:543.5,
			solutes:vec![(1,"C\u{2082}\u{2087}H\u{2082}\u{2089}NO\u{2081}\u{2081}".to_owned(),0)],
			solubility: 0.26,
			pka:vec![(7.34,"".to_owned()),(8.46,"".to_owned()),(9.46,"".to_owned())],  //actual 7.34,8.46,9.46 
			use_weak:true,
			salt:false,
			med: (true,20.0,90.0,"mg/m\u{20B2}".to_owned(),None),  //really damn hard to convert.
		 },
		 Compound{
			name:vec!["Epinephrine".to_owned()],
			formula:vec!["C\u{2089}H\u{2081}\u{2083}NO\u{2083}".to_owned(),"C9H13NO3".to_owned()],
			mmass:183.2,
			solutes:vec![(1,"C\u{2089}H\u{2081}\u{2083}NO\u{2083}".to_owned(),0)],
			solubility: 0.018,
			pka:vec![(8.59,"".to_owned())],  //but solubility is very low.
			use_weak:true,
			salt:false,
			med: (true,0.001,0.0001,"".to_owned(),None),  //CF
		 },
		 Compound{
			name:vec!["Ascorbic Acid".to_owned()],
			formula:vec!["C\u{2086}H\u{2088}O\u{2086}".to_owned(),"C6H8O6".to_owned()],
			mmass:176.1,
			solutes:vec![(1,"C\u{2086}H\u{2087}O\u{2086}".to_owned(),-1),(1,"H".to_owned(),1)],
			solubility: 33.0,
			pka:vec![(4.7,"C\u{2086}H\u{2087}O\u{2086}".to_owned())],
			use_weak:true,
			salt:false,
			med: (false,0.05,1.0,"".to_owned(),None),  //CF
		 },
		 Compound{
			name:vec!["Sodium Ascorbate".to_owned()],
			formula:vec!["C\u{2086}H\u{2087}O\u{2086}Na".to_owned(),"C6H7O6Na".to_owned(),"NaC6H7O6".to_owned()],
			mmass:198.11,
			solutes:vec![(1,"C\u{2086}H\u{2087}O\u{2086}".to_owned(),-1),(1,"Na".to_owned(),1)],
			solubility: 62.0,
			pka:vec![(4.7,"C\u{2086}H\u{2087}O\u{2086}".to_owned())],
			use_weak:true,
			salt:true,
			med: (false,0.056,1.13,"".to_owned(),None),  //CF
		 }
	];
	//Make extra compounds from config files and add to main library.							
	let mut extra_compounds = parse_compound_json();
	a.append(&mut extra_compounds);
	a
}


