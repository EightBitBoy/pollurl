from cli_test_helpers import shell


# https://github.com/alexliesenfeld/httpmock
# parallel execution of tests!

# TODO Test quiet results


BINARY_PATH = "target/debug/waiturl"


def run(argumentsAndOptions = ""):
  return shell(BINARY_PATH + " " + argumentsAndOptions)


# TODO Can the result code for missing arguments be configured?
def test_missing_mandatory_url():
  result = run()
  assert result.exit_code == 2
  assert "error: the following required arguments were not provided" in result.stderr

def test_cannot_parse_url():
  result = run("foobar")
  assert result.exit_code == 1
  assert "Error: The URL cannot be parsed!" in result.stdout

def test_success():
  result = run("https://httpbin.org/status/200")
  assert result.exit_code == 0
  assert "SUCCESS" in result.stdout

def test_success_quiet():
  result = run("https://httpbin.org/status/200 -q")
  assert result.exit_code == 0
  assert result.stdout == ""

def test_timeout_three_seconds():
  result = run("https://httpbin.org/status/404 -t 3")
  assert result.exit_code == 1
  assert "Timeout after 3 seconds!" in result.stdout

# def test_timeout_three_seconds_quiet():
#   result = run("https://httpbin.org/status/404 -t 3")
#   assert result.exit_code == 1
#   assert result.stdout == ""

# def test_failed to lookup address information: Name or service not known
