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
    pub Fiducial: Vec<Fiducial>,
    pub botpose_orb_wpiblue: [f64; 6],
    pub botpose_wpiblue: [f64; 6],
    pub botpose_tagcount: i32,

    pub stdev_mt1: [f64; 6],
}

#[derive(Deserialize, Clone)]
pub struct LimelightStatus {
    pub finalYaw: f64,
}

impl core::default::Default for LimelightResults {
    fn default() -> Self {
        Self {
            Fiducial: vec![],
            botpose_orb_wpiblue: [0.0; 6],
            botpose_wpiblue: [0.0; 6],
            botpose_tagcount: 0,

            stdev_mt1: [0.0; 6],
        }
    }
}

impl core::default::Default for LimelightStatus {
    fn default() -> Self {
        Self { finalYaw: 0.0 }
    }
}

#[derive(Deserialize, Clone)]
pub struct Fiducial {
    pub fID: i32,
    pub tx: f64,
    pub ty: f64,
    pub ta: f64,
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

    pub async fn response(&self) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("http://{}:{}/{}", self.ip.ip(), self.ip.port(), "results");

        println!("url: {:?}", url);

        let response: reqwest::Response = self
            .client
            .get(&url)
            .timeout(Duration::from_millis(100))
            .send()
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

    pub async fn status(&self) -> Result<LimelightStatus, LimelightError> {
        self.get_json("status").await
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
    use crate::limelight::LimelightStatus;

    #[test]
    fn parse_results() {
        let results = r#"{"Barcode":[],"Classifier":[],"Detector":[],"Fiducial":[{"ambig":0.16421737773136502,"fID":9,"fam":"36H11C","pts":[],"skew":[],"t6c_ts":[0.22155162144658755,0.8723182162333529,-1.1517094145376687,11.73683075410433,6.526105536697161,1.0714059898439148],"t6r_fs":[5.685235921616331,-0.25087151599249513,0.0,0.0,0.0,83.47268720158773],"t6r_fs_orb":[4.5383095703125,-1.7716643047332763,4.339218095950059e-09,0.0,0.0,0.0],"t6r_ts":[0.10459279002578253,1.131035350068871,-1.436558382843157,-178.5740274885974,83.47193994940375,-177.49331922764827],"t6t_cs":[-0.36718141864051584,-0.6226141243686383,1.2704283627244248,-11.691753058043192,-6.6073014025438255,0.2773175747438014],"t6t_rs":[-1.4097405548472688,-1.1329009551519176,-0.2881799396091251,170.7168580600749,-83.38698223082022,-170.4926140533997],"ta":0.008101089857518673,"tx":-16.595819597812422,"tx_nocross":-16.120444537341157,"txp":416.5489807128906,"ty":24.800034014654727,"ty_nocross":26.10864505149722,"typ":48.53244400024414},{"ambig":0.18536773136950546,"fID":10,"fam":"36H11C","pts":[],"skew":[],"t6c_ts":[-0.08294729786580057,0.8620224986371067,-1.129572619138643,11.751485401929942,4.312097898377248,0.16553662561175447],"t6r_fs":[5.667150250421209,-0.1845727903347258,0.0,0.0,0.0,85.68780820622915],"t6r_fs_orb":[4.1744208297729495,-1.4272932243347167,4.339218095950059e-09,0.0,0.0,0.0],"t6r_ts":[-0.18470850831191477,1.1226640705225905,-1.418473095491509,-178.03125688612107,85.68542642837001,-177.8601292669459],"t6t_cs":[-0.004703065478912138,-0.6135481087196067,1.2843029018690362,-11.771903811304444,-4.255340614687013,0.7173295056621084],"t6t_rs":[-1.4251858842599896,-1.1268907628014269,0.07429843404590039,177.8056683864845,-85.68480784098197,-177.65093179136497],"ta":0.007690894417464733,"tx":-0.6851869684906831,"tx_nocross":-0.20981190801941807,"txp":630.745361328125,"ty":24.226525365420045,"ty_nocross":25.535136402262538,"typ":58.7094841003418}],"PythonOut":[],"Retro":[],"botorient":{"alpha":0.001,"imumode":0,"interpbotyaw":0.0},"botpose":[5.6409874290856115,-0.19907903748432368,0.0,0.0,0.0,85.39996216809105],"botpose_avgarea":0.7895992137491703,"botpose_avgdist":1.442501610463488,"botpose_orb":[4.351532353483902,-1.5949051341754574,4.339218095950059e-09,0.0,0.0,0.0],"botpose_orb_wpiblue":[12.622032353483904,2.439594865824543,4.339218095950059e-09,0.0,0.0,0.0],"botpose_orb_wpired":[3.9189718787399626,5.629416681356967,4.339218095950059e-09,0.0,0.0,179.999847961155],"botpose_span":0.36285706826100567,"botpose_tagcount":2,"botpose_wpiblue":[13.91148742908561,3.8354209625156765,0.0,0.0,0.0,85.39996216809105],"botpose_wpired":[2.6295130991884754,4.233594006350564,0.0,0.0,0.0,-94.60018987139847],"cl":11.595222473144531,"fidx":102425,"focus_metric":0.0,"hw":{"cid":"9281","cpu":76.25178527832031,"dfree":1809,"dtot":2393,"hailo":{"power":0.0,"present":0,"temp":0.0,"throttle":0,"type":""},"ram":64.29114532470703,"temp":82.5999984741211},"hwtype":6,"ignorent":0,"imgsrc":0,"imu":{"data":[-18.786913187631736,-0.22851260376730667,-12.00128186530732,-18.786913187631736,-0.49000000953674316,-0.14000000059604645,-0.49000000953674316,-0.20788800716400146,0.0026839999482035637,0.9882000088691711],"quat":[0.6211248425000321,0.7665256597175586,-0.12807955902328444,-0.10118285222444114],"yaw":-18.786913187631736},"ntconnected":1,"pID":0.0,"pTYPE":"pipe_fiducial","rewind":{"bufferUsage":0.9999639391899109,"enabled":1,"flushing":0,"frameCount":22074,"latpen":948,"storedSeconds":1232.506959858},"stdev_mt1":[0.0012074207286287165,0.0010058804730128635,0.0,0.0,0.0,0.024404627198759573],"stdev_mt2":[0.00020573888543884913,0.001303226018535221,0.0,0.0,0.0,0.0],"t6c_rs":[0.079,-0.295,0.2617,0.0,11.9,-90.0],"ta":0.8098646998405457,"tl":64.05245971679688,"ts":5715334.351833,"ts_nt":3240461315,"ts_sys":1716852227210331,"ts_us":5709794252,"tx":-16.595819597812422,"txnc":-16.120444537341157,"ty":24.800034014654727,"tync":26.10864505149722,"uirefresh":0,"v":1}"#;

        let status = r#"{"cameraQuat":{"w":0.618324752282301,"x":0.7858487312471383,"y":0.004433830237560052,"z":-0.009829214705081038},"cid":9281,"cpu":74.0088119506836,"finalYaw":-0.2972398258824448,"finalimu":[-0.2972398258824448,1.1993825538249019,-13.603843080741768,-0.2972398258824448,-0.5249999761581421,-0.17499999701976776,-0.24500000476837158,-0.2408280074596405,-0.015371999703347683,0.9828320145606995],"fps":56.103553771972656,"hailoCount":1,"hailoPower":3.75,"hailoTemp":74.0,"hwType":6,"ignoreNT":0,"interfaceNeedsRefresh":0,"name":"","pipeImgCount":2,"pipelineIndex":0,"pipelineType":"pipe_fiducial","ram":34.71793746948242,"snapshotMode":0,"temp":84.80000305175781}"#;

        println!("{:?}", results);

        let results: LimelightResults = serde_json::from_str(results).unwrap();
        let status: LimelightStatus = serde_json::from_str(status).unwrap();

        // assert_eq!(results.tx, 0.3374213002711173);
        // assert_eq!(results.ty, -23.007593523896787);
        //assert_eq!(results.ta, 0.01910470798611641);
        assert_eq!(results.Fiducial[0].fID, 9);
        assert_eq!(results.Fiducial[0].tx, -16.595819597812422);
        assert_eq!(results.Fiducial[0].ty, 24.800034014654727);
        assert_eq!(results.Fiducial[0].ta, 0.008101089857518673);

        assert_eq!(results.botpose_tagcount, 2);
        assert_eq!(results.stdev_mt1[0], 0.0012074207286287165);

        assert_eq!(
            results.botpose_orb_wpiblue,
            [
                12.622032353483904,
                2.439594865824543,
                4.339218095950059e-09,
                0.0,
                0.0,
                0.0
            ]
        );

        assert_eq!(status.finalYaw, -0.2972398258824448);
    }
}
