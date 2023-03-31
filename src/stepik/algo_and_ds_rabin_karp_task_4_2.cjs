// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/description/
const rabin_karp_str_str = (t, p) => {

    if (p.length === 0 || t.length === 0 || p.length > t.length) {
        return []
    }
    const X = 263
    const MOD = 1000000007
    const powers_arr = [1]
    for (let i = 0; i < p.length; i++) {
        powers_arr.push((powers_arr[powers_arr.length - 1] % MOD * X % MOD + MOD) % MOD)
    }

    const hash = (str) => {
        let p = str.length - 1
        return [...str].reduce((acc, v) => {
                return (acc + (v.charCodeAt(0) % MOD * powers_arr[p--]) % MOD % MOD) % MOD
            }
            , 0)
    }

    const result = []
    const hash_p = hash(p)

    let hash_t = 0;
    for (let start = 0; start <= t.length - p.length; start++) {
        if (start === 0) {
            hash_t = hash(t.slice(0, p.length))
        } else {
            let sub_prev_char = (hash_t % MOD - (t[start - 1].charCodeAt(0) % MOD * powers_arr[p.length - 1]) % MOD + MOD) % MOD
            let add_last_char = t[start + p.length - 1].charCodeAt(0) % MOD
            hash_t = ((sub_prev_char % MOD * X % MOD) % MOD + add_last_char % MOD + MOD) % MOD
        }
        if (hash_p === hash_t) {
            result.push(start)
        }
    }

    return result
}

console.log(rabin_karp_str_str("hello", "ll")); // 2
console.log(rabin_karp_str_str("sadbutsad", "sad")); // 0, 6
console.log(rabin_karp_str_str("testTesttesT", "Test")); // 4
console.log(rabin_karp_str_str("aabaaabaaac", "aabaaac")); // 4
console.log(rabin_karp_str_str(Array(500).map(() => "a").join(), Array(300).map(() => "a").join())); // 4
console.log(rabin_karp_str_str(Array(500).map(() => "a").join(), Array(300).map(() => "a").join())); // 4
