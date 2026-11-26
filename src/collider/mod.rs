use std::sync::Arc;

use eframe::egui::util::id_type_map::TypeId;
use gluon::{
    ctx::GluonProcessContext,
    proc::{GluonError, GluonProcess},
};
use log::info;
use thiserror::Error;
use tokio::runtime::{self, Runtime};

use crate::BosonProcess;

pub struct Collider {
    client: reqwest::Client,
}

#[derive(Debug, Error)]
pub enum ColliderError {
    #[error("Failed to initalize collider!")]
    InitalizationFailed,
}

impl Collider {
    pub fn new() -> anyhow::Result<Self> {
        let client = reqwest::Client::new();
        Ok(Self { client })
    }

    fn run_collider(&mut self) -> Result<(), ColliderError> {
        todo!()
    }
}

#[async_trait::async_trait]
impl GluonProcess for Collider {
    async fn run(&mut self, mut ctx: GluonProcessContext) -> Result<(), GluonError> {
        tokio::select! {
            msg = ctx.recv() => {
                todo!()
            }

            else => {
                self.run_collider()?;
                Ok(())
            }
        }
    }
}
