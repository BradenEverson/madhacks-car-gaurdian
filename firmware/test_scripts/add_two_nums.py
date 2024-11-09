import sys

if len(sys.argv) < 3:
    print("Usage: python script.py <num1> <num2>")
    print(sys.argv)
    sys.exit(1)

try:
    num1 = int(sys.argv[1])
    num2 = int(sys.argv[2])
    
    result = num1 + num2
    print(result)
except ValueError:
    print("Both arguments must be numbers.")
    sys.exit(1)
