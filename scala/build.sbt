ThisBuild / scalaVersion     := "3.1.3" // Use the latest Scala 3 version compatible with ZIO
ThisBuild / version          := "0.1.0"
ThisBuild / organization     := "com.example"
ThisBuild / organizationName := "Example"

lazy val root = (project in file("."))
  .settings(
    name := "YourProjectName",
    libraryDependencies ++= Seq(
      "dev.zio" %% "zio" % "2.0.0" // Replace with the latest ZIO 2.x version compatible with Scala 3
    )
  )
