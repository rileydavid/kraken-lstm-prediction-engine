export interface Ohlc {
    bucket: Date;
    open_price?: number;
    high?: number;
    low?: number;
    close_price: number;
    volume?: number;
    count?: number;
    symbol?: string;
}
