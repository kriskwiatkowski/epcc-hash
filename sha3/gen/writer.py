
class Writer(object):
    ''' Context manager for writing assembly output to a file in a format required by the target language. '''
    def __init__(self, output_dir, language, filename):
        ''' Initialize the writer with the output directory, target language, and filename. 
        
        Args:
            output_dir (Path): The directory where the output file will be written.
            language (str): The target language for the output file. Can be "rust" or
                "generic" (for assembly).
            filename (str): The base name of the output file (without extension).
        '''
        self.output_dir = output_dir
        self.language = language
        if language == "rust":
            self.fname = filename + ".rs"
        else:
            self.fname = filename + ".s"

    def __enter__(self):
        self.fhandle = open(self.output_dir / self.fname, "w")
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        self.fhandle.close()
        return False

    def write(self, asm):
        ''' Write (append) the assembly output to the file in the appropriate format for the target language. 
        
        Args:
            asm: The assembly output to write.
        '''
        if self.language == "rust":
            self.fhandle.write(asm.rust_module)
        else:
            self.fhandle.write(asm.assembly)