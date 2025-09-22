use crate::java;
use bitvec::macros::internal::funty::Fundamental;
use jni::objects::{GlobalRef, JObject, JValue};
use jni::signature::{Primitive, ReturnType};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;
use thiserror::Error;

#[derive(Clone)]
pub struct Limelight {
    ip: SocketAddr,
    client: Client,
}

#[derive(Deserialize, Clone)]
pub struct LimelightResults {
    pub tx: f64,
    pub ty: f64,
    pub Fiducial: Vec<Fiducial>,
    pub botpose_orb_wpiblue: [f64; 6],
    pub botpose_wpiblue: [f64; 6],
}

impl core::default::Default for LimelightResults {
    fn default() -> Self {
        Self {
            tx: 0.0,
            ty: 0.0,
            Fiducial: vec![],
            botpose_orb_wpiblue: [0.0; 6],
            botpose_wpiblue: [0.0; 6],
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct Fiducial {
    pub fID: i32,
}

#[derive(Error, Debug)]
pub enum LimelightError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl Limelight {
    pub fn new(ip: SocketAddr) -> Self {
        Self {
            ip,
            client: Client::new(),
        }
    }

    // Limelightlib-rust
    async fn get_json<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, LimelightError> {
        let url = format!("http://{}:{}/{}", self.ip.ip(), self.ip.port(), endpoint);

        let response = self
            .client
            .get(&url)
            .timeout(Duration::from_millis(100))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    async fn post_json<T: serde::Serialize + ?Sized>(
        &self,
        endpoint: &str,
        data: &T,
    ) -> Result<bool, LimelightError> {
        let url = format!("http://{}:{}/{}", self.ip.ip(), self.ip.port(), endpoint);

        let response = self
            .client
            .post(&url)
            .json(data)
            .timeout(Duration::from_millis(100))
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    pub async fn results(&self) -> Result<LimelightResults, LimelightError> {
        self.get_json("results").await
    }

    pub async fn update_robot_orientation(&self, yaw: f64) -> Result<bool, LimelightError> {
        let orientation_data = vec![yaw, 0.0, 0.0, 0.0, 0.0, 0.0];
        self.post_json("update-robotorientation", &orientation_data)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::limelight::LimelightResults;

    #[test]
    fn parse_results() {
        let results = r#"{"Barcode":[],"Classifier":[],"Detector":[],"Fiducial":[{"fID":7,"fam":"36H11C","pts":[],"skew":[],"t6c_ts":[0.022673249781540734,-0.049669232184856756,-0.9686806051645971,18.740689266694265,-2.044208385329364,1.0270656893740011],"t6r_fs":[6.445014376461276,0.028245268630612295,0.0,0.0,0.0,2.0440467533570947],"t6r_fs_orb":[6.442269191741944,-0.012911224737763405,5.9604645663569045e-09,0.0,0.0,0.0],"t6r_ts":[0.02834703129926037,0.3504383472593898,-1.3285163388505807,-179.9907051465564,2.0443604009313363,-178.97293429793586],"t6t_cs":[0.012787976749228689,0.358687844929084,0.9013893283450441,-18.78185120050184,1.6053539610058427,-1.6299503586948567],"t6t_rs":[-0.012790547677344626,-0.35008950716477993,-1.328849106063408,179.9726391374379,-2.044198523887141,-178.97261198539164],"ta":0.01910470798611641,"tx":0.3374213002711173,"tx_nocross":0.8127963607423823,"txp":644.0413208007813,"ty":-23.007593523896787,"ty_nocross":-21.698982487054295,"typ":712.7247314453125}],"PythonOut":[],"Retro":[],"botpose":[6.445014376461276,0.028245268630612295,0.0,0.0,0.0,2.0440467533570947],"botpose_avgarea":1.910470798611641,"botpose_avgdist":0.9702181320216291,"botpose_orb":[6.442269191741944,-0.012911224737763405,5.9604645663569045e-09,0.0,0.0,0.0],"botpose_orb_wpiblue":[14.712794941741944,4.092588775262237,5.9604645663569045e-09,0.0,0.0,0.0],"botpose_orb_wpired":[1.828256592519149,4.11842831987705,5.9604645663569045e-09,0.0,0.0,179.99984796115504],"botpose_span":0.0,"botpose_tagcount":1,"botpose_wpiblue":[14.715540126461276,4.133745268630612,0.0,0.0,0.0,2.0440467533570947],"botpose_wpired":[1.825511298587369,4.077271833793268,0.0,0.0,0.0,-177.95610528613608],"cl":16.46666717529297,"focus_metric":0.0,"pID":0.0,"pTYPE":"pipe_fiducial","stdev_mt1":[0.0014506798277613892,0.005547281003364626,0.0,0.0,0.0,0.24007139832972604],"stdev_mt2":[0.00019906416892763364,2.6408903376396954e-06,null,0.0,0.0,0.0],"t6c_rs":[-0.36,0.0,0.4,0.0,18.75,-180.0],"ta":1.9114255905151367,"tl":49.093353271484375,"ts":8549005.248737,"tx":0.3374213002711173,"txnc":0.8127963607423823,"ty":-23.007593523896787,"tync":-21.698982487054295,"v":1}"#;

        println!("{:?}", results);

        let results: LimelightResults = serde_json::from_str(results).unwrap();
        assert_eq!(results.tx, 0.3374213002711173);
        assert_eq!(results.ty, -23.007593523896787);
        assert_eq!(results.Fiducial[0].fID, 7);
        assert_eq!(
            results.botpose_orb_wpiblue,
            [
                14.712794941741944,
                4.092588775262237,
                5.9604645663569045e-09,
                0.0,
                0.0,
                0.0
            ]
        );
    }
}
