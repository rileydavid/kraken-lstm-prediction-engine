export interface Trade {
    time: Date;
    price: number;
    volume: number;
    side?: string;
    order_type?: string;
    symbol_id: number;
}
