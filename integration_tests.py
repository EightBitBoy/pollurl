from cli_test_helpers import shell

def func(x):
  return x + 1

def test_answer():
  assert func(4) == 5

def test_example():
  result = shell("echo 'hello'")
  assert result.exit_code == 0
  assert "hell" in result.stdout
