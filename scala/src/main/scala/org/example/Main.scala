import zio._

object MyApp extends ZIOAppDefault {
  def run: URIO[Any, ExitCode] = {
    myAppLogic.exitCode
  }

  val myAppLogic: ZIO[Any, Throwable, Unit] = 
    for {
      _ <- Console.printLine("Hello, World!")
      _ <- Console.printLine("Welcome to ZIO 2 with Scala 3!")
    } yield ()
}

