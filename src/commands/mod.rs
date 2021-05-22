/*
    Copyright 2021 Volt Contributors

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

// Std Imports
use std::str::FromStr;
use std::sync::Arc;

// Library Imports
use anyhow::Result;
use async_trait::async_trait;

// Crate Level Imports
use crate::utils::App;

// Modules
pub mod add;
pub mod clone;
pub mod create;
pub mod deploy;
pub mod help;
pub mod init;
pub mod install;
pub mod migrate;
pub mod remove;
#[derive(Debug)]
pub enum AppCommand {
    Add,
    Help,
    Init,
    Install,
    Remove,
    Deploy,
    Clone,
    Create,
    Migrate,
}

impl FromStr for AppCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(Self::Add),
            "help" => Ok(Self::Help),
            "init" => Ok(Self::Init),
            "install" => Ok(Self::Install),
            "remove" => Ok(Self::Remove),
            "deploy" => Ok(Self::Deploy),
            "clone" => Ok(Self::Clone),
            "create" => Ok(Self::Create),
            "migrate" => Ok(Self::Migrate),
            _ => Err(()),
        }
    }
}

impl AppCommand {
    pub fn current() -> Option<Self> {
        match std::env::args().nth(1) {
            Some(cmd) => Self::from_str(cmd.as_str()).ok(),
            None => None,
        }
    }

    pub fn help(&self) -> String {
        match self {
            Self::Add => add::Add::help(),
            Self::Help => help::Help::help(),
            Self::Init => init::Init::help(),
            Self::Install => install::Install::help(),
            Self::Remove => remove::Remove::help(),
            Self::Deploy => deploy::Deploy::help(),
            Self::Clone => clone::Clone::help(),
            Self::Create => create::Create::help(),
            Self::Migrate => migrate::Migrate::help(),
        }
    }

    pub async fn run(&self, app: App) -> Result<()> {
        let app = Arc::new(app);
        match self {
            Self::Add => add::Add::exec(app).await,
            Self::Help => help::Help::exec(app).await,
            Self::Init => init::Init::exec(app).await,
            Self::Install => install::Install::exec(app).await,
            Self::Remove => remove::Remove::exec(app).await,
            Self::Deploy => deploy::Deploy::exec(app).await,
            Self::Clone => clone::Clone::exec(app).await,
            Self::Create => create::Create::exec(app).await,
            Self::Migrate => migrate::Migrate::exec(app).await,
        }
    }
}

#[async_trait]
pub trait Command {
    fn help() -> String;

    async fn exec(app: Arc<App>) -> Result<()>;
}
