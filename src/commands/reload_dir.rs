use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::preview;
use crate::window::JoshutoView;

#[derive(Clone, Debug)]
pub struct ReloadDirList;

impl ReloadDirList {
    pub fn new() -> Self {
        ReloadDirList
    }
    pub const fn command() -> &'static str {
        "reload_dir_list"
    }

    pub fn reload(context: &mut JoshutoContext, view: &JoshutoView) -> Result<(), std::io::Error> {
        let curr_tab = &mut context.tabs[context.curr_tab_index];
        curr_tab.reload_contents(&context.config_t.sort_option)?;
        curr_tab.refresh(
            view,
            &context.config_t,
            &context.username,
            &context.hostname,
        );
        Ok(())
    }
}

impl JoshutoCommand for ReloadDirList {}

impl std::fmt::Display for ReloadDirList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for ReloadDirList {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        match Self::reload(context, view) {
            Ok(_) => {}
            Err(e) => return Err(JoshutoError::IO(e)),
        }
        preview::preview_file(
            &mut context.tabs[context.curr_tab_index],
            view,
            &context.config_t,
        );
        ncurses::doupdate();
        Ok(())
    }
}
