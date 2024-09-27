import ctypes


def main():
    # Load the shared library
    lib = ctypes.CDLL('./build/libdata.so')  # Change to 'data.dll' on Windows

    # Define the return types and argument types for the functions
    lib.createData.restype = ctypes.POINTER(ctypes.c_void_p)
    lib.destroyData.argtypes = [ctypes.POINTER(ctypes.c_void_p)]

    # 调用C++函数
    # Create a Data instance
    data_instance = lib.createData(ctypes.c_int(8899))

    # 调用
    lib.doSomething()

    # Call the function to display the value (assuming you have a way to do this)
    my_data_ptr = ctypes.cast(data_instance, ctypes.POINTER(ctypes.c_void_p))
    lib.destroyData(data_instance)

    print("Data instance created and destroyed successfully.")

    pass

if __name__ == "__main__":
    main()