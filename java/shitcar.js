
// ETH USDC
// USDT USDC

const mul = 2000
const pairs = {
    'ETH': 'USDC',
    'USDT': 'USDC',
}

function getBalance(k, wallet) {
    return 10 * k == 'ETH' ? mul : 1
}

function selectForSwap(wallet) {
    const getB = (x) => getBalance(x, wallet)

    const all = {}
    Object.entries(pairs).forEach(([k, v]) => {
        all[k] = all[k] || []
        all[v] = all[v] || []
        all[k].push([v, [k, v]])
        all[v].push([k, [k, v]])
    })

    const mx = Object.keys(all)
        .sort((a, b) => getB(b) - getB(a))[0]

    return all[mx]
        .filter(x => x[1][0] == mx || x[1][1] == mx)
        .sort((a, b) => getB(a[1]) - getB(b[1]))[0][1]
}