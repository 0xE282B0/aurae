/* -------------------------------------------------------------------------- *\
 *             Apache 2.0 License Copyright © 2022 The Aurae Authors          *
 *                                                                            *
 *                +--------------------------------------------+              *
 *                |   █████╗ ██╗   ██╗██████╗  █████╗ ███████╗ |              *
 *                |  ██╔══██╗██║   ██║██╔══██╗██╔══██╗██╔════╝ |              *
 *                |  ███████║██║   ██║██████╔╝███████║█████╗   |              *
 *                |  ██╔══██║██║   ██║██╔══██╗██╔══██║██╔══╝   |              *
 *                |  ██║  ██║╚██████╔╝██║  ██║██║  ██║███████╗ |              *
 *                |  ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝ |              *
 *                +--------------------------------------------+              *
 *                                                                            *
 *                         Distributed Systems Runtime                        *
 *                                                                            *
 * -------------------------------------------------------------------------- *
 *                                                                            *
 *   Licensed under the Apache License, Version 2.0 (the "License");          *
 *   you may not use this file except in compliance with the License.         *
 *   You may obtain a copy of the License at                                  *
 *                                                                            *
 *       http://www.apache.org/licenses/LICENSE-2.0                           *
 *                                                                            *
 *   Unless required by applicable law or agreed to in writing, software      *
 *   distributed under the License is distributed on an "AS IS" BASIS,        *
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. *
 *   See the License for the specific language governing permissions and      *
 *   limitations under the License.                                           *
 *                                                                            *
\* -------------------------------------------------------------------------- */

#![allow(unused)]
#![allow(clippy::module_inception)]

use anyhow::{Context, Result};
use aurae_client::{runtime::pod_service::PodServiceClient, AuraeClient};
use aurae_proto::runtime::{
    pod_service_server, Pod, PodServiceAllocateRequest,
    PodServiceAllocateResponse, PodServiceFreeRequest, PodServiceFreeResponse,
    PodServiceStartRequest, PodServiceStartResponse, PodServiceStopRequest,
    PodServiceStopResponse,
};
use libcontainer::{
    container::builder::ContainerBuilder, syscall::syscall::create_syscall,
};
use liboci_cli::Run;
use std::path::PathBuf;
use tonic::{Request, Response, Status};
use tracing::info;

#[derive(Debug, Clone)]
pub struct PodService {
    // These are used for the cache as in the cells/executables
    root_path: PathBuf,
    //pods: Arc<Mutex<Pods>>,
    //containers: Arc<Mutex<Containers>>,
}

impl PodService {
    pub fn new(root_path: PathBuf) -> Self {
        PodService { root_path }
    }
}

#[tonic::async_trait]
impl pod_service_server::PodService for PodService {
    async fn allocate(
        &self,
        request: Request<PodServiceAllocateRequest>,
    ) -> Result<Response<PodServiceAllocateResponse>, Status> {
        let request = request.into_inner();
        let pod = request.pod.expect("pod");
        let name = pod.name;
        let image = pod.image;
        info!(
            "PodService: allocate() name={:?} image={:?}",
            name.clone(),
            image.clone()
        );

        let _container_bundle = bundle::container::container(&image);

        // Hack in from: https://github.com/containers/youki/blob/main/crates/youki/src/commands/run.rs

        let syscall = create_syscall();
        let mut container = ContainerBuilder::new(name, syscall.as_ref())
            // .with_pid_file(args.pid_file.as_ref())?
            // .with_console_socket(args.console_socket.as_ref())
            .with_root_path(self.root_path.join("bundles"))
            .expect("root path")
            .as_init("examples/busybox.oci/busybox") // TODO Implement the download and un-tar logic for container images
            .with_systemd(false)
            .build()
            .expect("build");

        container.start(); // TODO cache the container and move to start()

        Ok(Response::new(PodServiceAllocateResponse {}))
    }
    async fn free(
        &self,
        request: Request<PodServiceFreeRequest>,
    ) -> Result<Response<PodServiceFreeResponse>, Status> {
        let _request = request.into_inner();

        // TODO Destroy container
        // TODO Destroy /var/run/bundles/<name>

        Ok(Response::new(PodServiceFreeResponse {}))
    }
    async fn start(
        &self,
        request: Request<PodServiceStartRequest>,
    ) -> Result<Response<PodServiceStartResponse>, Status> {
        let _request = request.into_inner();
        Ok(Response::new(PodServiceStartResponse {}))
    }
    async fn stop(
        &self,
        request: Request<PodServiceStopRequest>,
    ) -> Result<Response<PodServiceStopResponse>, Status> {
        let _request = request.into_inner();
        Ok(Response::new(PodServiceStopResponse {}))
    }
}
