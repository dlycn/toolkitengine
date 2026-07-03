import asyncio

async def say_hello():
    print("Hello, World!")
    await asyncio.sleep(1)
    print("Hello, again!")

async def main():
    await say_hello()

asyncio.run(main())
