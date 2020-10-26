use diesel::prelude::*;
use diesel::pg::expression::dsl::any;
use crate::diesel::RunQueryDsl;

use serde::{Serialize, Deserialize};

//use crate::juniper::{Executor, FieldResult};
//use crate::juniper::{Executor, FieldResult};
//use juniper_eager_loading::{prelude::*, EagerLoading, HasOne};

use crate::schema::{myapp_prestation,myapp_agenda,myapp_breakdowntype,myapp_pathology};

use crate::schema::myapp_breakdowntype::dsl::*;
use crate::schema::myapp_pathology::dsl::*;


//use juniper_eager_loading::LoadFrom;


use crate::graphql::Context;

use chrono::{DateTime, Utc};


#[derive(Clone,Associations,Eq,PartialEq, Queryable,QueryableByName,Serialize, Deserialize, Debug, Identifiable)]
#[belongs_to(Prestation,foreign_key = "prestation_id")]
#[table_name = "myapp_agenda"]
#[primary_key(id)]
pub struct Agenda {
    pub id: i32,
    pub agendaNumber: i32,
    pub contactPerson: Option<String>,
    pub dateAgendaCreation: DateTime<Utc>,
    pub mediaTypeAgenda: String,
    pub levelAgenda: String,
    pub telephoneNumber: String,
    pub dateAgendaTodo: DateTime<Utc>,
    pub commentsAgenda: Option<String>,
    pub historyAgenda: Option<String>,
    pub dateAgendaClose: Option<DateTime<Utc>>,
    pub activityAgenda: String,
    pub agendaCase_id: String,
    pub agendaPersonAssigned_id: i32,
    pub prestation_id: i32,
    pub agendaUser_id: i32,
}


//#[derive(Eq,PartialEq, Queryable,Insertable,Serialize, Deserialize, Debug, Identifiable)]
#[derive(Clone,Eq,PartialEq, Queryable,Insertable,Serialize, Deserialize, Debug, Identifiable)]
#[table_name = "myapp_prestation"]
#[primary_key(id)]
pub struct Prestation {
    pub id: i32,
    pub libellePrestation: String,
}

impl juniper_eager_loading::LoadFrom<i32> for Prestation{
    type Error = diesel::result::Error;
    type Context = Context;
    
    fn load(
        ids:&[i32],
        _field_args :&(),
        ctx: &Context,
    ) -> Result<Vec<Self>, Self::Error>{
        use crate::schema::myapp_prestation::dsl::*;
        
        myapp_prestation.filter(id.eq(any(ids))).load::<Prestation>(&ctx.connexion)
    
    }

}


#[derive(Eq,PartialEq, Queryable,QueryableByName,Insertable,Serialize, Deserialize, Debug, Identifiable)]
#[table_name = "myapp_pathology"]
#[primary_key(pathology_id)]
pub struct MyappPathology {
    pub pathology_id: String,
    pub type_of_pathology: String,
    pub name_of_pathology: String,
}

#[derive(Eq,PartialEq, Queryable,Insertable,Serialize, Deserialize, Debug, Identifiable)]
#[table_name = "myapp_pathology"]
#[primary_key(pathology_id)]
pub struct NewMyappPathology {
    pub pathology_id: String,
    pub type_of_pathology: String,
    pub name_of_pathology: String,
}


impl MyappPathology{

    pub fn list(connection : &PgConnection) -> Vec<MyappPathology>{
        let resultat = myapp_pathology
        .filter(type_of_pathology.eq("D"))
        .order_by(type_of_pathology)
        .order_by(pathology_id.desc())
        .load::<MyappPathology>(connection)
        .expect("Error");
        
        return resultat;
        
    }


    pub fn show(pathology: &str, connection: &PgConnection) -> Vec<MyappPathology>{
        let resultat =  myapp_pathology
            .find (pathology)
            .load::<MyappPathology>(connection)
            .expect("error loading item");

        return resultat;
    }
    
    pub fn all(connection : &PgConnection) -> Vec<MyappPathology>{
        
        let resultat =  myapp_pathology
            .load::<MyappPathology>(connection)
            .expect("error loading items");
        return resultat;    
    }
    
    
}



#[derive(Eq,PartialEq, Queryable,Insertable, Serialize, Deserialize, Debug, Identifiable)]
#[table_name = "myapp_breakdowntype"]
pub struct MyappBreakdowntype {
    pub id: i32,
    pub breakdown: String,
}

impl MyappBreakdowntype{

    pub fn list(connection : &PgConnection) -> Vec<MyappBreakdowntype>{
         let resultat = myapp_breakdowntype
        .filter(id.eq(2))
        .order_by(id)
        .order_by(breakdown.desc())
        .load::<MyappBreakdowntype>(connection)
        .expect("rror");
        
        return resultat;
        
    }


    pub fn show(id_breakdowntype: &i32, connection: &PgConnection) -> Vec<MyappBreakdowntype>{
        let resultat =  myapp_breakdowntype
            .find (id_breakdowntype)
            .load::<MyappBreakdowntype>(connection)// <MyappPathology>(&conn)
            .expect("error loading item");

        return resultat;
    }
    
    pub fn all(connection : &PgConnection) -> Vec<MyappBreakdowntype>{
        
        let resultat =  myapp_breakdowntype
            .load::<MyappBreakdowntype>(connection)
            .expect("error loading items");
        
        return resultat;    
    }
}
