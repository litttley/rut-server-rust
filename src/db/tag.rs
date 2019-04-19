// tag typed model and msg handler

use actix::{ Handler, Message };
use diesel::prelude::*;
use diesel::{ self, QueryDsl, ExpressionMethods, dsl::any, PgTextExpressionMethods, RunQueryDsl };
use chrono::{ Local, NaiveDateTime, Utc, Duration };
use uuid::Uuid;

use crate::Dba;
use crate::errors::ServiceError;
use crate::db::msg::{ Msg, TagMsg, TagListMsg };
use crate::schema::{ tags, tagruts, tagitems, tagetcs, startags };


// use to build select query
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable,Insertable)]
#[table_name="tags"]
pub struct Tag {
    pub id: String,
    pub tname: String,
    pub intro: String,
    pub logo: String,
    pub pname: String,  // parent tag name != tname. check constain
    pub item_count: i32,
    pub rut_count: i32,
    pub etc_count: i32,
    pub star_count: i32,
    pub vote: i32,       //cal per star,rut,item,comment
}

// Rut's constructor
impl Tag {
    pub fn new(tname: String) -> Self {
        Tag {
            id: tname.clone(),
            tname: tname.clone(),
            intro: "".to_owned(),
            logo: "".to_owned(),
            pname: "".to_owned(),
            item_count: 0,
            rut_count: 0,
            etc_count: 0,
            star_count: 0,
            vote: 0,
        }
    }
}

// as msg in create new tag, get tag
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CheckTag {
    pub tname: String,
    pub action: String, // get / post / delete
}

impl Message for CheckTag {
    type Result = Result<TagMsg, ServiceError>;
}

// handle msg from api::tag.new_tag and get_tag
impl Handler<CheckTag> for Dba {
    type Result = Result<TagMsg, ServiceError>;

    fn handle(&mut self, tg: CheckTag, _: &mut Self::Context) -> Self::Result {
        use crate::schema::tags::dsl::*;
        let conn = &self.0.get().unwrap();
        
        let action = tg.action.trim();
        if action == "POST" {
            let newtag = Tag::new(tg.tname);
            let tag_new = diesel::insert_into(tags)
                .values(&newtag)
                .get_result::<Tag>(conn)?;

            Ok( TagMsg { 
                status: 200, 
                message: "Added".to_string(),
                tag: tag_new.clone(),
            })
        } else { // GET
            let tag_q = tags.filter(&tname.eq(&tg.tname))
                .get_result::<Tag>(conn)?;

            Ok( TagMsg { 
                status: 200, 
                message: "Get".to_string(),
                tag: tag_q.clone(),
            })
        }
    }
}

// as msg in query tag list
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum QueryTags {
    RutID(String),
    ItemID(String),
    TagID(String),
    UserID(String),
    Index(String),
}

impl Message for QueryTags {
    type Result = Result<TagListMsg, ServiceError>;
}

// handle msg from api::tag.get_tag_list
impl Handler<QueryTags> for Dba {
    type Result = Result<TagListMsg, ServiceError>;

    fn handle(&mut self, per: QueryTags, _: &mut Self::Context) -> Self::Result {
        use crate::schema::tags::dsl::*;
        let conn = &self.0.get().unwrap();
        
        let mut tag_list: Vec<String> = Vec::new();
        
        match per {
            QueryTags::RutID(r) => {
                use crate::schema::tagruts::dsl::*;
                tag_list = tagruts.filter(&rut_id.eq(&r)).select(tname)
                    .order(count.desc()).limit(10) // order per count
                    .load::<String>(conn)?;
            },
            QueryTags::ItemID(i) => {
                use crate::schema::tagitems::dsl::*;
                tag_list = tagitems.filter(&item_id.eq(&i)).select(tname)
                    .order(count.desc()).limit(10) 
                    .load::<String>(conn)?;
            },
            QueryTags::TagID(t) => {
                tag_list = tags.filter(&pname.eq(&t)).select(tname)
                    .order(vote.desc()).limit(10) 
                    .load::<String>(conn)?;
            },
            QueryTags::UserID(u) => { 
                use crate::schema::startags::dsl::*;
                tag_list = startags.filter(&uname.eq(&u)).select(tname)
                    .order(star_at.desc()).limit(42) 
                    .load::<String>(conn)?;
            },
            QueryTags::Index(_) => {
                tag_list = tags.select(tname).order(vote.desc()).limit(16)
                    .load::<String>(conn)?;
            },
        }

        Ok( TagListMsg { 
            status: 200, 
            message: "Success".to_string(),
            tags: tag_list.clone(),
            count: tag_list.len(),
        })
    }
}

// as msg in update tag
#[derive(Deserialize,Serialize,Debug,Clone,AsChangeset)]
#[table_name="tags"]
pub struct UpdateTag {
    pub tname: String,
    pub intro: String,
    pub logo: String,
    pub pname: String,  // parent tag name
}

impl Message for UpdateTag {
    type Result = Result<TagMsg, ServiceError>;
}

// handle msg from api::tag.update_tag
impl Handler<UpdateTag> for Dba {
    type Result = Result<TagMsg, ServiceError>;

    fn handle(&mut self, tg: UpdateTag, _: &mut Self::Context) -> Self::Result {
        use crate::schema::tags::dsl::*;
        let conn = &self.0.get().unwrap();

        let tag_update = diesel::update(tags.filter(&tname.eq(&tg.tname)))
            .set((
                intro.eq(tg.intro.clone()),
                logo.eq(tg.logo.clone()),
                pname.eq(tg.pname.clone()),  // to check if pname existing?
            ))
            .get_result::<Tag>(conn)?;

        Ok( TagMsg { 
            status: 201, 
            message: "Updated".to_string(),
            tag: tag_update.clone(),
        })
    }
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable,Insertable)]
#[table_name="tagruts"]
pub struct TagRut {
    pub id: String,
    pub tname: String,
    pub rut_id: String,
    pub count: i32,
}

// as msg in tag or untag rut
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RutTag {
    pub tnames: Vec<String>,
    pub rut_id: String,
    pub action: u8, // tag 1 or untag 0
}

impl Message for RutTag {
    type Result = Result<Msg, ServiceError>;
}

// handle msg from api::tag.tag_rut :  to be optimized
impl Handler<RutTag> for Dba {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, rutg: RutTag, _: &mut Self::Context) -> Self::Result {
        use crate::schema::tagruts::dsl::*;
        let conn = &self.0.get().unwrap();

        let action = rutg.action;
        let rutID = rutg.rut_id;

        if action == 1 {  // tag
            for rtg in rutg.tnames {
                // to check if tagged with a same tag
                let tr = tagruts.filter(&tname.eq(&rtg)).filter(&rut_id.eq(&rutID))
                    .load::<TagRut>(conn)?.pop();
                match tr {
                    // if tagged, update count + 1 in tagruts
                    Some(tgr) => {
                        diesel::update(&tgr).set(count.eq(count + 1)).execute(conn)?;
                    },
                    // else new tag-rut
                    None => {
                        let new_tag_rut = TagRut {
                            id: (rtg.clone() + "-" + &rutID),
                            tname: rtg.clone(),
                            rut_id: rutID.clone(),
                            count: 1,
                        };
                        //  to check if tname in tags? otherwise, new_tag
                        use crate::schema::tags::dsl::*;
                        let tag_check = tags.filter(&tname.eq(&rtg)).load::<Tag>(conn)?.pop();
                        match tag_check {
                            // if existing, tag then rut_count + 1 in tags
                            Some(t) => {
                                diesel::insert_into(tagruts).values(&new_tag_rut).execute(conn)?;
                                // then update tags.rut_count
                                diesel::update(&t).set(rut_count.eq(rut_count + 1)).execute(conn)?;
                            },
                            // if no existing tname, new_tag
                            None => {
                                let newtag = Tag {
                                    id: rtg.clone(),
                                    tname: rtg.clone(),
                                    intro: "".to_owned(),
                                    logo: "".to_owned(),
                                    pname: "".to_owned(),
                                    item_count: 0,
                                    rut_count: 1,
                                    etc_count: 0,
                                    star_count: 0,
                                    vote: 0,
                                };
                                // new_tag 
                                diesel::insert_into(tags).values(&newtag).execute(conn)?;
                                // then tag_rut
                                diesel::insert_into(tagruts).values(&new_tag_rut).execute(conn)?;
                            },
                        }  
                    },
                }
            }
        } else { // untag
            for rtg in rutg.tnames {
                diesel::delete(tagruts.filter(&tname.eq(&rtg))).execute(conn)?;
            }
        }

        Ok( Msg { 
            status: 201, 
            message: "Done".to_string(),
        })
    }
}


#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable,Insertable)]
#[table_name="startags"]
pub struct StarTag {
    pub id: String,
    pub uname: String,
    pub tname: String,
    pub star_at: NaiveDateTime,
    pub note: String,
}

// as msg in star or unstar tag
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StarOrTag {
    pub uname: String,
    pub tname: String,
    pub note: String,
    pub action: u8,  // 0- unstar, 1- star
}

impl Message for StarOrTag {
    type Result = Result<Msg, ServiceError>;
}

// as msg to check if star a tag
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StarTagStatus {
    pub uname: String,
    pub tname: String,
}

impl Message for StarTagStatus {
    type Result = Result<Msg, ServiceError>;
}

// handle msg from api::tag.star_unstar_tag
impl Handler<StarOrTag> for Dba {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, act: StarOrTag, _: &mut Self::Context) -> Self::Result {
        use crate::schema::startags::dsl::*;
        let conn = &self.0.get().unwrap();
        
        match act.action {
            1  => {  // star
                // limit star tag to 42
                let tag_star_num = 
                    startags.filter(&uname.eq(&act.uname)).count().execute(conn)?;
                if tag_star_num > 42 {
                    return Ok( Msg{ status: 418, message: "unstar".to_string(),})
                }
                
                let uid = format!("{}", uuid::Uuid::new_v4());
                let new_star = StarTag {
                    id: uid,
                    uname: act.clone().uname,
                    tname: act.clone().tname,
                    star_at: Utc::now().naive_utc(),
                    note: act.clone().note,
                };
                diesel::insert_into(startags).values(&new_star).execute(conn)?;
                // to update star_count + 1 in tag
                use crate::schema::tags::dsl::{tags, tname, star_count, rut_count, vote};
                diesel::update(tags.filter(&tname.eq(&act.tname)))
                    .set((
                        star_count.eq(star_count + 1),
                        vote.eq(rut_count * 2 + star_count)  // cal vote, to be task
                    ))
                    .execute(conn)?;

                Ok( Msg{ status: 200, message: "star".to_string(),})
            },
            0 => { // unsatr
                diesel::delete(
                    startags.filter(&tname.eq(&act.tname))
                            .filter(&uname.eq(&act.uname))
                )
                .execute(conn)?;

                // to update the star_count - 1 in tag
                use crate::schema::tags::dsl::{tags, tname as t_name, star_count};
                diesel::update(tags.filter(&t_name.eq(&act.tname)))
                    .set(star_count.eq(star_count - 1)).execute(conn)?;

                Ok( Msg{ status: 200, message: "unstar".to_string(),})
            },
            _ =>  { Ok( Msg{ status: 400, message: "unstar".to_string(),}) },
        }
    }
}

// handle msg from api::tag.star_tag_status
impl Handler<StarTagStatus> for Dba {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, status: StarTagStatus, _: &mut Self::Context) -> Self::Result {
        use crate::schema::startags::dsl::*;
        let conn = &self.0.get().unwrap();

        let check_status = startags
            .filter(&uname.eq(&status.uname))
            .filter(&tname.eq(&status.tname))
            .load::<StarTag>(conn)?.pop();
        
        match check_status {
            Some(_) => { Ok( Msg{status: 200, message: "star".to_string() }) },
            None => { Ok( Msg{ status: 200, message: "unstar".to_string() }) },
        }
    }
}

// to do
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="tagitems"]
pub struct TagItem {
    pub id: String,
    pub tname: String,
    pub item_id: String,
    pub count: i32,
}

// to do
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Identifiable,Queryable)]
#[table_name="tagetcs"]
pub struct TagEtc {
    pub id: String,
    pub tname: String,
    pub etc_id: String,
}
