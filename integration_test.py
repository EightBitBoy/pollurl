from cli_test_helpers import shell


BINARY_PATH = "target/debug/waiturl"


def run(argumentsAndOptions = ""):
  return shell(BINARY_PATH + " " + argumentsAndOptions)

def test_example():
  result = shell("echo 'Hello, World!'")
  assert result.exit_code == 0
  assert "lo" in result.stdout

# TODO Can the result code for missing arguments be configured?
def test_missing_mandatory_url():
  result = run()
  assert result.exit_code == 2

def test_success():
  result = run("https://httpbin.org/status/200")
  assert result.exit_code == 0
  assert "SUCCESS" in result.stdout

# def test_invalid_url
# def test_failed to lookup address information: Name or service not known
