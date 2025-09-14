export function thousandComma(s: string | number) {
    return String(s).replace(/\B(?=(\d{3})+(?!\d))/g, ',')
}

export function fmtNumber(p: number | string, precisionLen: number = 5) {
    const str = String(p)
    const [whole, decimal] = str.split('.')
    if (!decimal || whole.length >= precisionLen) return thousandComma(whole)
    if (whole === '0') {
        let startDecimalCount = false
        let decimalCount = 0
        let decimalSliceIndex = 0
        for (const c of decimal) {
            decimalSliceIndex++
            if (c !== '0') startDecimalCount = true
            if (startDecimalCount) decimalCount++
            if (decimalCount >= precisionLen) break
        }
        return `0.${decimal.slice(0, decimalSliceIndex)}`
    }
    const decimalLen = precisionLen - whole.length
    return `${thousandComma(whole)}.${decimal.slice(0, decimalLen)}`
}

export function fmtPercent(p: number) {
    return p.toFixed(2) + '%'
}
