//Anything related to POST requests for collaborators and it's properties goes here.
use super::Collaborator;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Collaborator Create
///
/// Create a new collaborator.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#collaborator-create)
pub struct CollaboratorCreate {
    /// app_id can be the app name or the app id
    pub app_id: String,
    /// The parameters to pass to the Heroku API
    pub params: CollaboratorCreateParams,
}

/// Create build with parameters.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#collaborator-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct CollaboratorCreateParams {
    /// unique email address, identifier of an account or Implicit reference to currently authorized user
    pub user: String,
    /// whether to suppress email invitation when creating collaborator
    pub silent: Option<bool>,
}

impl HerokuEndpoint<Collaborator, (), CollaboratorCreateParams> for CollaboratorCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/collaborators", self.app_id)
    }
    fn body(&self) -> Option<CollaboratorCreateParams> {
        Some(self.params.clone())
    }
}