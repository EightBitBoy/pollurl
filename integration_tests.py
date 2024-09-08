from cli_test_helpers import shell

def test_example():
  result = shell("echo 'Hello, World!'")
  assert result.exit_code == 0
  assert "lo" in result.stdout
