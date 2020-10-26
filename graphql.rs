use crate::db::PostgresPool;

use diesel::pg::PgConnection;
use crate::diesel::RunQueryDsl;


use crate::juniper::{Executor, FieldResult};
use juniper_eager_loading::{prelude::*, EagerLoading, HasOne};
//use juniper_from_schema::QueryTrail;
use juniper_from_schema::graphql_schema_from_file;
graphql_schema_from_file!("src/graphql_schema.graphql");

use crate::schema::myapp_agenda;
//use crate::schema::myapp_prestation;


pub struct Context {
    pub connexion: PgConnection,
}

// This impl allows us to pass in GraphQLContext as the Context for GraphQL
// objects
impl juniper::Context for Context {}



#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Agenda {
    
    agenda : crate::models::Agenda,

    // these are the defaults. `#[has_one(default)]` would also work here.
    #[has_one(
        foreign_key_field = prestation_id,
        root_model_field = prestation,
        graphql_field = prestation
    )]
    prestation: HasOne<Prestation>,
}

impl AgendaFields for Agenda {
    fn field_id(&self, executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.agenda.id)
    }

    fn field_prestation(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, Prestation, Walked>,
    ) -> FieldResult<&Prestation> {
        self.prestation.try_unwrap().map_err(From::from)
    }
    fn field_agenda_number(&self, _: &juniper::Executor<Context>) -> std::result::Result<&i32, juniper::FieldError> { 
         todo!() }
    fn field_contact_person(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<std::string::String>, juniper::FieldError> { todo!() }
    fn field_date_agenda_creation(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<DateTime>, juniper::FieldError> { todo!() }
    fn field_media_type_agenda(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<std::string::String>, juniper::FieldError> { todo!() }
    fn field_level_agenda(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<std::string::String>, juniper::FieldError> { todo!() }
    fn field_telephone_number(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<std::string::String>, juniper::FieldError> { todo!() }
    fn field_date_agenda_todo(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<DateTime>, juniper::FieldError> { todo!() }
    fn field_comments_agenda(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<std::string::String>, juniper::FieldError> { todo!() }
    fn field_date_agenda_close(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<DateTime>, juniper::FieldError> { todo!() }
    fn field_history_agenda(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<std::string::String>, juniper::FieldError> { todo!() }
    fn field_activity_agenda(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::option::Option<std::string::String>, juniper::FieldError> { todo!() }
    fn field_agenda_case_id(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { todo!() }
    fn field_agenda_person_assigned_id(&self, _: &juniper::Executor<Context>) -> std::result::Result<&i32, juniper::FieldError> { todo!() }
    fn field_agenda_user_id(&self, _: &juniper::Executor<Context>) -> std::result::Result<&i32, juniper::FieldError> { todo!() }
}

#[derive(Clone, EagerLoading)]
#[eager_loading(context = Context, error = diesel::result::Error)]
pub struct Prestation {
    prestation: crate::models::Prestation,
}

impl PrestationFields for Prestation {
    fn field_id(&self, executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.prestation.id)
    }
    fn field_libelle_prestation(&self, _: &juniper::Executor<Context>) -> std::result::Result<&std::string::String, juniper::FieldError> { 
        todo!() }
}


pub struct Query;

impl QueryFields for Query{
    fn field_agendas(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_,Agenda, Walked>,
    ) -> FieldResult<Vec<Agenda>>{
        let ctx = executor.context();
        let agenda_models = myapp_agenda::table.load::<crate::models::Agenda>(&ctx.connexion)?;
        let mut agendas = Agenda::from_db_models(&agenda_models);
        Agenda::eager_load_all_children_for_each(&mut agendas,&agenda_models,ctx,trail)?;
        Ok(agendas)

    } 

    
}