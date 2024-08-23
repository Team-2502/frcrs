package frc.robot;

import com.ctre.phoenix.motorcontrol.ControlMode;
import com.ctre.phoenix6.configs.CurrentLimitsConfigs;
import com.kauailabs.navx.frc.AHRS;
import com.revrobotics.CANSparkMax;
import com.revrobotics.CANSparkLowLevel.MotorType;
import static com.revrobotics.CANSparkMax.IdleMode;
import com.revrobotics.CANSparkFlex;
import com.ctre.phoenix6.StatusSignal;
import static com.revrobotics.CANSparkMax.ControlType;

import edu.wpi.first.apriltag.AprilTagFieldLayout;
import edu.wpi.first.apriltag.AprilTagFields;
import edu.wpi.first.hal.AllianceStationID;
import edu.wpi.first.hal.DriverStationJNI;
import edu.wpi.first.math.geometry.*;
import edu.wpi.first.networktables.NetworkTableInstance;
import edu.wpi.first.wpilibj.smartdashboard.Field2d;
import edu.wpi.first.wpilibj.smartdashboard.SendableChooser;
import edu.wpi.first.wpilibj.smartdashboard.SmartDashboard;
import edu.wpi.first.wpilibj.GenericHID.RumbleType;
import org.photonvision.PhotonCamera;
import org.photonvision.PhotonPoseEstimator;
import org.photonvision.PhotonPoseEstimator.PoseStrategy;
import org.photonvision.EstimatedRobotPose;

import java.io.OutputStream;
import java.net.HttpURLConnection;
import java.net.URLConnection;
import java.nio.charset.StandardCharsets;
import java.util.Optional;
import java.net.URL;

import com.ctre.phoenix6.hardware.TalonFX;

import static edu.wpi.first.wpilibj.RobotBase.isReal;

/// Wrapper to use rev classes in rust, for whatever reason they cannot be accesed from rust directly
public class Wrapper {
    public static MotorType kBrushless() {
        return MotorType.kBrushless;
    }
    public static MotorType kBrushed() {
        return MotorType.kBrushed;
    }

    public static IdleMode kBrake() { return IdleMode.kBrake; }
    public static IdleMode kCoast() { return IdleMode.kCoast; }

    public static ControlType kPosition() { return ControlType.kPosition; }
    public static ControlType kVelocity() { return ControlType.kVelocity; }

    public static CANSparkFlex createSparkFlex(int id) { return new CANSparkFlex(id, MotorType.kBrushless); }

    public static void sparkFollow(CANSparkMax leader, CANSparkMax follower, boolean invert) { follower.follow(leader, invert); }

    public static int getAllianceStation() {
        AllianceStationID allianceID = DriverStationJNI.getAllianceStation();
        switch (allianceID) {
        case Blue1:
            return 4;
        case Blue2:
            return 5;
        case Blue3:
            return 6;
        case Red1:
            return 1;
        case Red2:
            return 2;
        case Red3:
            return 3;
        default:
            return 0;
        }
    }

    public static void startNetworkTables() {
        NetworkTableInstance inst = NetworkTableInstance.getDefault();
        if (isReal()) {
            inst.startServer("/home/lvuser/networktables.json");
        } else {
            inst.startServer();
        }
    }

    public static AHRS createAHRS() {
        return new AHRS();
    }

    public static double getAngle(AHRS navx) {
        return navx.getAngle();
    }

    public static ControlMode ctreVelocity() {
        return ControlMode.Velocity;
    }

    public static ControlMode ctrePosition() {
        return ControlMode.Position;
    }

    public static ControlMode ctrePercent() {
      return ControlMode.PercentOutput;
    }

    public static double ctre6GetVelocity(com.ctre.phoenix6.hardware.TalonFX motor) {
      return motor.getVelocity().getValue();
    }

    public static RumbleType kBothRumble() {
        return RumbleType.kBothRumble;
    }

    public static RumbleType kLeftRumble() {
        return RumbleType.kLeftRumble;
    }

    public static RumbleType kRightRumble() {
        return RumbleType.kRightRumble;
    }

    public static double getValue(StatusSignal<Double> holder) {
        return holder.getValue();
    }

    /*public static void setSpeed(TalonFX motor, double speed) {
      motor.set(ControlMode.PercentOutput, speed);
    }*/

    //public static TalonFXInvertType TalonFXCounterClockwise() { return TalonFXInvertType.CounterClockwise; }

    public static SendableChooser<Integer> autoChooser() {
        SendableChooser<Integer> chooser = new SendableChooser<>();
        chooser.addOption("pass on right", 1);
        chooser.setDefaultOption("top", 4);
        chooser.addOption("eco", 3);
        chooser.addOption("test", 2);

        SmartDashboard.putData(chooser);
        return chooser;
    }

    private static final Field2d m_field = new Field2d();

    public static void setPosition(double x, double y, double theta) {
        m_field.setRobotPose(x, y, Rotation2d.fromRadians(theta));

        SmartDashboard.putData("field", m_field);
    }

    static AprilTagFieldLayout aprilTagFieldLayout = AprilTagFields.k2024Crescendo.loadAprilTagLayoutField();

    static PhotonCamera cam = new PhotonCamera("Global_Shutter_Camera");
    static Transform3d robotToCam = new Transform3d(new Translation3d(-0.5, 0.0, 0.5), new Rotation3d(0,0,180)); //Cam mounted facing forward, half a meter forward of center, half a meter up from center.
    static PhotonPoseEstimator photonPoseEstimator = new PhotonPoseEstimator(aprilTagFieldLayout, PoseStrategy.LOWEST_AMBIGUITY, cam, robotToCam);

    static Pose2d prevEstimatedRobotPose = new Pose2d();

    public static void updateVisionOdo() {
        //photonPoseEstimator.setReferencePose(prevEstimatedRobotPose);
        Optional<EstimatedRobotPose> robotPose =  photonPoseEstimator.update();

        if (robotPose.isPresent()) {
            try {
                // URL to which you want to send the POST request
                URL url = new URL("http://10.25.2.2:5807/set_position");

                //prevEstimatedRobotPose = robotPose.get().estimatedPose.toPose2d();

                // JSON data to be sent in the request body
                String jsonData = "{\"x\": " + robotPose.get().estimatedPose.getX() +
                        ", \"y\": " + robotPose.get().estimatedPose.getY() + ", \"theta\": " +
                        (robotPose.get().estimatedPose.getRotation().getZ() + Math.PI) + "}";

                System.out.println(jsonData);

                // Open connection
                HttpURLConnection connection = (HttpURLConnection) url.openConnection();

                // Set request method
                connection.setRequestMethod("POST");

                // Set headers
                connection.setRequestProperty("Content-Type", "application/json");

                // Enable output for the request body
                connection.setDoOutput(true);

                // Write JSON data to the output stream
                try (OutputStream outputStream = connection.getOutputStream()) {
                    byte[] input = jsonData.getBytes("utf-8");
                    outputStream.write(input, 0, input.length);
                }

                int responseCode = connection.getResponseCode();

                //System.out.println("Response Code: " + responseCode);

                connection.disconnect();
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }
}
