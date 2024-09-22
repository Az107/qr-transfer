import json

class Chunk():
    def __init__(self, data, part: int, total: int) -> None:
        self.data = data
        self.part = part
        self.total = total

    def __str__(self) -> str:
        return json.dumps(self.__dict__, ensure_ascii=False)

class BigChunkus:
    def __get_total_chunks(self) -> int:
        data_len = len(self.data.encode("utf-8"))
        return (data_len + self.chunk_size - 1) // self.chunk_size

    def __len__(self) -> int:
        return self.__get_total_chunks()

    def __init__(self,data: str, chunk_size = 3000) -> None:
        self.pointer = 0
        self.chunk_size = chunk_size # chunk size in bytes
        self.data = data
        self.total_chunks = self.__get_total_chunks()

    def __iter__(self):
        self.pointer = 0
        return self

    def __next__(self) -> Chunk:
        if self.pointer >= self.total_chunks:
            raise StopIteration

        start = self.pointer * self.chunk_size
        end = start + self.chunk_size
        byte_data = self.data.encode("utf-8")[start:end]
        chunk_data = byte_data.decode("utf-8", errors="ignore")

        chunk_obj = Chunk(chunk_data, self.pointer + 1, self.total_chunks)
        self.pointer += 1
        return chunk_obj
