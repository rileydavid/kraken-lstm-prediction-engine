export class Trade {
    
    private time;
    private price;
    private volume;
    private side;
    private order_type;
    private symbol;
   
    constructor(time: String, price: String, volume: String, side: String, order_type: String, symbol: String){
        this.time = time;
        this.price = price;
        this.volume = volume;
        this.side = side;
        this.order_type = order_type;
        this.symbol = symbol;
    }

}