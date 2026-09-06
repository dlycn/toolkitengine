
async function testFetch() {
    try {
        const r = await fetch('https://httpbin.org/get');
        console.log('fetch 成功:', await r.json());
    } catch (e) {
        console.error('fetch 失败:', e);
    }
}
testFetch();