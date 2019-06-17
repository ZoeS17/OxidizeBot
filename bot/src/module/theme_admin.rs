use crate::{auth, command, db, module};

pub struct Handler<'a> {
    pub themes: &'a db::Themes,
}

impl<'a> command::Handler for Handler<'a> {
    fn handle(&mut self, ctx: &mut command::Context<'_, '_>) -> Result<(), failure::Error> {
        let next = command_base!(ctx, self.themes, "theme", ThemeEdit);

        match next.as_ref().map(String::as_str) {
            Some("edit") => {
                ctx.check_scope(auth::Scope::ThemeEdit)?;

                let name = ctx_try!(ctx.next_str("<name> <track-id>"));
                let track_id = ctx_try!(ctx.next_parse("<name> <track-id>"));

                self.themes.edit(ctx.user.target, &name, track_id)?;
                ctx.respond("Edited theme.");
            }
            Some("edit-duration") => {
                ctx.check_scope(auth::Scope::ThemeEdit)?;

                let name = ctx_try!(ctx.next_str("<name> <start> <end>"));
                let start = ctx_try!(ctx.next_parse("<name> <start> <end>"));
                let end = ctx_try!(ctx.next_parse_optional());

                self.themes
                    .edit_duration(ctx.user.target, &name, start, end)?;
                ctx.respond("Edited theme.");
            }
            None | Some(..) => {
                ctx.respond(
                    "Expected: show, list, edit, edit-duration, delete, enable, disable, or group.",
                );
            }
        }

        Ok(())
    }
}

pub struct Module;

impl Module {
    pub fn load() -> Self {
        Module
    }
}

impl super::Module for Module {
    fn ty(&self) -> &'static str {
        "theme"
    }

    fn hook(
        &self,
        module::HookContext {
            handlers, themes, ..
        }: module::HookContext<'_, '_>,
    ) -> Result<(), failure::Error> {
        handlers.insert("theme", Handler { themes });
        Ok(())
    }
}
