use super::*;
use math_util::{round, Feq, SQRT_2, FRAC_PI_4};
use rstar::Point as RStarPoint;
use crate::{Point, Points, Coordinate, read_wkt};

#[test]
fn test_linestring() {
    let wkt_str = "LINESTRING ( 197.00410980408563 222.70222340571223, 200.00410980408563 223.70222340571223, 201.00410980408563 222.70222340571223, 202.00410980408563 219.70222340571223, 202.00410980408563 218.70222340571223, 204.00410980408563 219.70222340571223, 207.00410980408563 219.70222340571223, 208.00410980408563 218.70222340571223, 211.00410980408563 217.70222340571223, 212.00410980408563 217.70222340571223, 213.00410980408563 217.70222340571223, 214.00410980408563 217.70222340571223, 217.00410980408563 218.70222340571223, 219.00410980408563 217.70222340571223, 220.00410980408563 217.70222340571223, 220.00410980408563 220.70222340571223, 220.00410980408563 223.70222340571223, 221.00410980408563 224.70222340571223, 221.00410980408563 227.70222340571223, 222.00410980408563 227.70222340571223, 224.00410980408563 229.70222340571223, 224.00410980408563 231.70222340571223, 225.00410980408563 231.70222340571223, 226.00410980408563 234.70222340571223, 225.00410980408563 231.70222340571223, 225.00410980408563 230.70222340571223, 226.00410980408563 230.70222340571223, 226.00410980408563 231.70222340571223, 228.00410980408563 233.70222340571223, 226.00410980408563 235.70222340571223, 223.00410980408563 236.70222340571223, 222.00410980408563 238.70222340571223, 222.00410980408563 239.70222340571223, 223.00410980408563 238.70222340571223, 225.00410980408563 236.70222340571223, 228.00410980408563 236.70222340571223, 229.00410980408563 236.70222340571223, 230.00410980408563 238.70222340571223, 231.00410980408563 240.70222340571223, 231.00410980408563 242.70222340571223, 232.00410980408563 244.70222340571223, 231.00410980408563 246.70222340571223, 231.00410980408563 247.70222340571223, 231.00410980408563 248.70222340571223, 230.00410980408563 248.70222340571223, 229.00410980408563 246.70222340571223, 230.00410980408563 245.70222340571223, 230.00410980408563 242.70222340571223, 229.00410980408563 242.70222340571223, 228.00410980408563 240.70222340571223, 226.00410980408563 240.70222340571223, 226.00410980408563 242.70222340571223, 228.00410980408563 242.70222340571223, 226.00410980408563 242.70222340571223, 228.00410980408563 244.70222340571223, 228.00410980408563 246.70222340571223, 225.00410980408563 248.70222340571223, 229.00410980408563 247.70222340571223, 229.00410980408563 248.70222340571223, 230.00410980408563 249.70222340571223, 229.00410980408563 250.70222340571223, 226.00410980408563 251.70222340571223, 225.00410980408563 250.70222340571223, 222.00410980408563 252.70222340571223, 222.00410980408563 255.70222340571223, 222.00410980408563 257.70222340571223, 219.00410980408563 259.70222340571223, 218.00410980408563 259.70222340571223, 218.00410980408563 258.70222340571223, 217.00410980408563 257.70222340571223, 215.00410980408563 257.70222340571223, 214.00410980408563 257.70222340571223, 215.00410980408563 257.70222340571223, 217.00410980408563 259.70222340571223, 218.00410980408563 259.70222340571223, 215.00410980408563 260.70222340571223, 213.00410980408563 259.70222340571223, 212.00410980408563 259.70222340571223, 212.00410980408563 260.70222340571223, 212.00410980408563 262.70222340571223, 212.00410980408563 263.70222340571223, 214.00410980408563 263.70222340571223, 214.00410980408563 266.70222340571223, 214.00410980408563 267.70222340571223, 214.00410980408563 268.70222340571223, 214.00410980408563 270.7022234057123, 214.00410980408563 271.7022234057123, 218.00410980408563 274.7022234057122, 217.00410980408563 275.7022234057122, 215.00410980408563 279.7022234057122, 217.00410980408563 280.7022234057122, 218.00410980408563 280.7022234057122, 218.00410980408563 281.7022234057122, 215.00410980408563 280.7022234057122, 215.00410980408563 281.7022234057122, 214.00410980408563 281.7022234057122, 215.00410980408563 283.7022234057122, 217.00410980408563 284.7022234057122, 217.00410980408563 286.7022234057122, 218.00410980408563 290.7022234057122, 217.00410980408563 294.7022234057122, 219.00410980408563 294.7022234057122, 217.00410980408563 299.7022234057122, 215.00410980408563 300.7022234057122, 214.00410980408563 302.7022234057122, 213.00410980408563 305.7022234057122, 213.00410980408563 308.7022234057122, 210.00410980408563 314.7022234057122, 209.00410980408563 315.7022234057122, 208.00410980408563 315.7022234057122, 208.00410980408563 316.7022234057122, 209.00410980408563 317.7022234057122, 209.00410980408563 316.7022234057122, 209.00410980408563 318.7022234057122, 210.00410980408563 319.7022234057122, 209.00410980408563 322.7022234057122, 208.00410980408563 322.7022234057122, 209.00410980408563 321.7022234057122, 207.00410980408563 322.7022234057122, 207.00410980408563 321.7022234057122, 204.00410980408563 322.7022234057122, 202.00410980408563 319.7022234057122, 200.00410980408563 321.7022234057122, 201.00410980408563 319.7022234057122, 200.00410980408563 319.7022234057122, 199.00410980408563 319.7022234057122, 200.00410980408563 321.7022234057122, 199.00410980408563 321.7022234057122, 198.00410980408563 322.7022234057122, 197.00410980408563 323.7022234057122, 198.00410980408563 322.7022234057122, 198.00410980408563 321.7022234057122, 198.00410980408563 319.7022234057122, 197.00410980408563 319.7022234057122, 197.00410980408563 317.7022234057122, 197.00410980408563 319.7022234057122, 197.00410980408563 321.7022234057122, 196.00410980408563 323.7022234057122, 193.00410980408563 322.7022234057122, 195.00410980408563 321.7022234057122, 193.00410980408563 321.7022234057122, 193.00410980408563 322.7022234057122, 192.00410980408563 322.7022234057122, 191.00410980408563 322.7022234057122, 187.00410980408563 322.7022234057122, 185.00410980408563 323.7022234057122, 184.00410980408563 323.7022234057122, 182.00410980408563 323.7022234057122, 184.00410980408563 324.7022234057122, 185.00410980408563 325.7022234057122, 184.00410980408563 325.7022234057122, 184.00410980408563 326.7022234057122, 180.00410980408563 326.7022234057122, 180.00410980408563 328.7022234057122, 179.00410980408563 328.7022234057122, 178.00410980408563 328.7022234057122, 177.00410980408563 327.7022234057122, 177.00410980408563 328.7022234057122, 176.00410980408563 328.7022234057122, 177.00410980408563 329.7022234057122, 175.00410980408563 330.7022234057122, 174.00410980408563 332.7022234057122, 170.00410980408563 333.7022234057122, 169.00410980408563 332.7022234057122, 170.00410980408563 330.7022234057122, 171.00410980408563 329.7022234057122, 168.00410980408563 330.7022234057122, 166.00410980408563 329.7022234057122, 166.00410980408563 330.7022234057122, 167.00410980408563 330.7022234057122, 168.00410980408563 332.7022234057122, 168.00410980408563 333.7022234057122, 166.00410980408563 335.7022234057122, 165.00410980408563 335.7022234057122, 164.00410980408563 335.7022234057122, 163.00410980408563 335.7022234057122, 162.00410980408563 336.7022234057122, 162.00410980408563 338.7022234057122, 162.00410980408563 337.7022234057122, 159.00410980408563 337.7022234057122, 158.00410980408563 340.7022234057122, 157.00410980408563 340.7022234057122, 157.00410980408563 338.7022234057122, 155.00410980408563 338.7022234057122, 155.00410980408563 340.7022234057122, 154.00410980408563 340.7022234057122, 152.00410980408563 338.7022234057122, 151.00410980408563 340.7022234057122, 149.00410980408563 338.7022234057122, 149.00410980408563 340.7022234057122, 148.00410980408563 340.7022234057122, 148.00410980408563 339.7022234057122, 148.00410980408563 341.7022234057122, 147.00410980408563 341.7022234057122, 146.00410980408563 340.7022234057122, 146.00410980408563 341.7022234057122, 145.00410980408563 341.7022234057122, 144.00410980408563 340.7022234057122, 145.00410980408563 338.7022234057122, 144.00410980408563 337.7022234057122, 144.00410980408563 338.7022234057122, 143.00410980408563 338.7022234057122, 142.00410980408563 338.7022234057122, 142.00410980408563 340.7022234057122, 141.00410980408563 340.7022234057122, 140.00410980408563 340.7022234057122, 140.00410980408563 338.7022234057122, 137.00410980408563 340.7022234057122, 138.00410980408563 340.7022234057122, 136.00410980408563 340.7022234057122, 136.00410980408563 339.7022234057122, 135.00410980408563 340.7022234057122, 135.00410980408563 339.7022234057122, 137.00410980408563 337.7022234057122, 140.00410980408563 337.7022234057122, 142.00410980408563 335.7022234057122, 136.00410980408563 337.7022234057122, 135.00410980408563 337.7022234057122, 136.00410980408563 336.7022234057122, 137.00410980408563 335.7022234057122, 144.00410980408563 334.7022234057122, 144.00410980408563 333.7022234057122, 143.00410980408563 332.7022234057122, 143.00410980408563 330.7022234057122, 143.00410980408563 332.7022234057122, 142.00410980408563 334.7022234057122, 134.00410980408563 334.7022234057122, 134.00410980408563 335.7022234057122, 132.00410980408563 335.7022234057122, 131.00410980408563 335.7022234057122, 130.00410980408563 335.7022234057122, 130.00410980408563 334.7022234057122, 131.00410980408563 335.7022234057122, 131.00410980408563 334.7022234057122, 131.00410980408563 333.7022234057122, 133.00410980408563 333.7022234057122, 134.00410980408563 332.7022234057122, 133.00410980408563 332.7022234057122, 133.00410980408563 330.7022234057122, 135.00410980408563 329.7022234057122, 135.00410980408563 330.7022234057122, 135.00410980408563 329.7022234057122, 137.00410980408563 329.7022234057122, 136.00410980408563 329.7022234057122, 138.00410980408563 327.7022234057122, 141.00410980408563 327.7022234057122, 142.00410980408563 327.7022234057122, 140.00410980408563 327.7022234057122, 137.00410980408563 328.7022234057122, 136.00410980408563 328.7022234057122, 135.00410980408563 328.7022234057122, 130.00410980408563 330.7022234057122, 130.00410980408563 329.7022234057122, 129.00410980408563 329.7022234057122, 129.00410980408563 328.7022234057122, 130.00410980408563 327.7022234057122, 129.00410980408563 326.7022234057122, 127.00410980408563 327.7022234057122, 126.00410980408563 328.7022234057122, 126.00410980408563 326.7022234057122, 125.00410980408563 326.7022234057122, 125.00410980408563 324.7022234057122, 127.00410980408563 324.7022234057122, 129.00410980408563 324.7022234057122, 127.00410980408563 323.7022234057122, 129.00410980408563 322.7022234057122, 134.00410980408563 319.7022234057122, 135.00410980408563 318.7022234057122, 135.00410980408563 319.7022234057122, 136.00410980408563 317.7022234057122, 137.00410980408563 317.7022234057122, 140.00410980408563 317.7022234057122, 136.00410980408563 316.7022234057122, 135.00410980408563 316.7022234057122, 135.00410980408563 317.7022234057122, 134.00410980408563 316.7022234057122, 131.00410980408563 317.7022234057122, 130.00410980408563 316.7022234057122, 129.00410980408563 316.7022234057122, 127.00410980408563 316.7022234057122, 129.00410980408563 317.7022234057122, 126.00410980408563 316.7022234057122, 127.00410980408563 317.7022234057122, 125.00410980408563 317.7022234057122, 124.00410980408563 316.7022234057122, 124.00410980408563 315.7022234057122, 125.00410980408563 315.7022234057122, 124.00410980408563 314.7022234057122, 125.00410980408563 314.7022234057122, 125.00410980408563 315.7022234057122, 126.00410980408563 315.7022234057122, 126.00410980408563 313.7022234057122, 127.00410980408563 313.7022234057122, 129.00410980408563 312.7022234057122, 131.00410980408563 312.7022234057122, 131.00410980408563 313.7022234057122, 133.00410980408563 314.7022234057122, 134.00410980408563 312.7022234057122, 133.00410980408563 311.7022234057122, 134.00410980408563 310.7022234057122, 134.00410980408563 312.7022234057122, 135.00410980408563 314.7022234057122, 137.00410980408563 314.7022234057122, 140.00410980408563 314.7022234057122, 140.00410980408563 313.7022234057122, 136.00410980408563 313.7022234057122, 136.00410980408563 311.7022234057122, 137.00410980408563 312.7022234057122, 137.00410980408563 311.7022234057122, 138.00410980408563 308.7022234057122, 136.00410980408563 307.7022234057122, 141.00410980408563 306.7022234057122, 142.00410980408563 305.7022234057122, 143.00410980408563 306.7022234057122, 143.00410980408563 305.7022234057122, 142.00410980408563 305.7022234057122, 143.00410980408563 303.7022234057122, 144.00410980408563 303.7022234057122, 146.00410980408563 304.7022234057122, 146.00410980408563 303.7022234057122, 149.00410980408563 304.7022234057122, 155.00410980408563 302.7022234057122, 155.00410980408563 303.7022234057122, 156.00410980408563 303.7022234057122, 157.00410980408563 302.7022234057122, 159.00410980408563 302.7022234057122, 162.00410980408563 302.7022234057122, 158.00410980408563 301.7022234057122, 157.00410980408563 301.7022234057122, 157.00410980408563 300.7022234057122, 156.00410980408563 299.7022234057122, 155.00410980408563 299.7022234057122, 155.00410980408563 300.7022234057122, 152.00410980408563 303.7022234057122, 151.00410980408563 302.7022234057122, 149.00410980408563 304.7022234057122, 148.00410980408563 303.7022234057122, 149.00410980408563 302.7022234057122, 147.00410980408563 302.7022234057122, 145.00410980408563 302.7022234057122, 145.00410980408563 301.7022234057122, 144.00410980408563 300.7022234057122, 145.00410980408563 302.7022234057122, 142.00410980408563 302.7022234057122, 142.00410980408563 303.7022234057122, 137.00410980408563 302.7022234057122, 136.00410980408563 302.7022234057122, 140.00410980408563 302.7022234057122, 143.00410980408563 299.7022234057122, 144.00410980408563 297.7022234057122, 146.00410980408563 297.7022234057122, 146.00410980408563 295.7022234057122, 147.00410980408563 294.7022234057122, 147.00410980408563 292.7022234057122, 149.00410980408563 291.7022234057122, 148.00410980408563 291.7022234057122, 146.00410980408563 291.7022234057122, 147.00410980408563 290.7022234057122, 151.00410980408563 285.7022234057122, 154.00410980408563 285.7022234057122, 155.00410980408563 285.7022234057122, 157.00410980408563 285.7022234057122, 155.00410980408563 285.7022234057122, 157.00410980408563 285.7022234057122, 158.00410980408563 285.7022234057122, 158.00410980408563 284.7022234057122, 159.00410980408563 283.7022234057122, 158.00410980408563 283.7022234057122, 157.00410980408563 283.7022234057122, 157.00410980408563 282.7022234057122, 158.00410980408563 281.7022234057122, 158.00410980408563 280.7022234057122, 156.00410980408563 280.7022234057122, 154.00410980408563 281.7022234057122, 147.00410980408563 281.7022234057122, 146.00410980408563 281.7022234057122, 146.00410980408563 279.7022234057122, 145.00410980408563 281.7022234057122, 145.00410980408563 280.7022234057122, 145.00410980408563 279.7022234057122, 145.00410980408563 278.7022234057122, 146.00410980408563 278.7022234057122, 145.00410980408563 277.7022234057122, 145.00410980408563 278.7022234057122, 145.00410980408563 277.7022234057122, 144.00410980408563 278.7022234057122, 143.00410980408563 278.7022234057122, 142.00410980408563 277.7022234057122, 141.00410980408563 277.7022234057122, 141.00410980408563 275.7022234057122, 143.00410980408563 274.7022234057122, 141.00410980408563 274.7022234057122, 140.00410980408563 274.7022234057122, 140.00410980408563 275.7022234057122, 137.00410980408563 274.7022234057122, 136.00410980408563 273.7022234057123, 135.00410980408563 274.7022234057122, 135.00410980408563 273.7022234057123, 134.00410980408563 273.7022234057123, 135.00410980408563 271.7022234057123, 137.00410980408563 272.7022234057123, 136.00410980408563 271.7022234057123, 136.00410980408563 270.7022234057123, 135.00410980408563 269.70222340571223, 134.00410980408563 269.70222340571223, 136.00410980408563 269.70222340571223, 136.00410980408563 268.70222340571223, 140.00410980408563 269.70222340571223, 137.00410980408563 266.70222340571223, 142.00410980408563 266.70222340571223, 142.00410980408563 264.70222340571223, 142.00410980408563 262.70222340571223, 144.00410980408563 261.70222340571223, 146.00410980408563 262.70222340571223, 147.00410980408563 261.70222340571223, 146.00410980408563 261.70222340571223, 147.00410980408563 260.70222340571223, 146.00410980408563 260.70222340571223, 147.00410980408563 259.70222340571223, 147.00410980408563 258.70222340571223, 144.00410980408563 258.70222340571223, 143.00410980408563 258.70222340571223, 142.00410980408563 258.70222340571223, 142.00410980408563 256.70222340571223, 144.00410980408563 257.70222340571223, 144.00410980408563 255.70222340571223, 144.00410980408563 256.70222340571223, 144.00410980408563 255.70222340571223, 143.00410980408563 252.70222340571223, 143.00410980408563 251.70222340571223, 144.00410980408563 251.70222340571223, 143.00410980408563 251.70222340571223, 144.00410980408563 250.70222340571223, 143.00410980408563 250.70222340571223, 143.00410980408563 251.70222340571223, 142.00410980408563 250.70222340571223, 143.00410980408563 249.70222340571223, 142.00410980408563 249.70222340571223, 143.00410980408563 248.70222340571223, 142.00410980408563 248.70222340571223, 143.00410980408563 247.70222340571223, 141.00410980408563 246.70222340571223, 141.00410980408563 247.70222340571223, 141.00410980408563 248.70222340571223, 140.00410980408563 248.70222340571223, 140.00410980408563 249.70222340571223, 140.00410980408563 250.70222340571223, 138.00410980408563 250.70222340571223, 140.00410980408563 246.70222340571223, 138.00410980408563 245.70222340571223, 140.00410980408563 245.70222340571223, 142.00410980408563 242.70222340571223, 142.00410980408563 245.70222340571223, 143.00410980408563 246.70222340571223, 143.00410980408563 247.70222340571223, 142.00410980408563 246.70222340571223, 144.00410980408563 247.70222340571223, 144.00410980408563 245.70222340571223, 144.00410980408563 246.70222340571223, 144.00410980408563 245.70222340571223, 144.00410980408563 244.70222340571223, 144.00410980408563 242.70222340571223, 146.00410980408563 244.70222340571223, 151.00410980408563 244.70222340571223, 152.00410980408563 246.70222340571223, 154.00410980408563 245.70222340571223, 155.00410980408563 245.70222340571223, 155.00410980408563 246.70222340571223, 156.00410980408563 247.70222340571223, 156.00410980408563 248.70222340571223, 157.00410980408563 249.70222340571223, 159.00410980408563 246.70222340571223, 162.00410980408563 246.70222340571223, 164.00410980408563 248.70222340571223, 165.00410980408563 247.70222340571223, 166.00410980408563 248.70222340571223, 168.00410980408563 249.70222340571223, 169.00410980408563 249.70222340571223, 168.00410980408563 248.70222340571223, 169.00410980408563 247.70222340571223, 170.00410980408563 247.70222340571223, 169.00410980408563 246.70222340571223, 170.00410980408563 246.70222340571223, 169.00410980408563 245.70222340571223, 168.00410980408563 246.70222340571223, 167.00410980408563 245.70222340571223, 169.00410980408563 244.70222340571223, 169.00410980408563 242.70222340571223, 171.00410980408563 242.70222340571223, 170.00410980408563 241.70222340571223, 171.00410980408563 240.70222340571223, 171.00410980408563 241.70222340571223, 175.00410980408563 241.70222340571223, 176.00410980408563 239.70222340571223, 176.00410980408563 238.70222340571223, 178.00410980408563 236.70222340571223, 179.00410980408563 236.70222340571223, 178.00410980408563 236.70222340571223, 176.00410980408563 236.70222340571223, 175.00410980408563 236.70222340571223, 174.00410980408563 236.70222340571223, 171.00410980408563 238.70222340571223, 171.00410980408563 236.70222340571223, 174.00410980408563 236.70222340571223, 171.00410980408563 236.70222340571223, 169.00410980408563 236.70222340571223, 168.00410980408563 236.70222340571223, 165.00410980408563 234.70222340571223, 166.00410980408563 233.70222340571223, 165.00410980408563 233.70222340571223, 166.00410980408563 233.70222340571223, 166.00410980408563 231.70222340571223, 168.00410980408563 230.70222340571223, 173.00410980408563 231.70222340571223, 171.00410980408563 230.70222340571223, 170.00410980408563 230.70222340571223, 171.00410980408563 230.70222340571223, 174.00410980408563 231.70222340571223, 170.00410980408563 228.70222340571223, 171.00410980408563 228.70222340571223, 175.00410980408563 228.70222340571223, 174.00410980408563 227.70222340571223, 175.00410980408563 227.70222340571223, 176.00410980408563 227.70222340571223, 175.00410980408563 227.70222340571223, 174.00410980408563 226.70222340571223, 174.00410980408563 227.70222340571223, 173.00410980408563 226.70222340571223, 173.00410980408563 224.70222340571223, 175.00410980408563 224.70222340571223, 174.00410980408563 224.70222340571223, 173.00410980408563 222.70222340571223, 174.00410980408563 222.70222340571223, 174.00410980408563 220.70222340571223, 175.00410980408563 222.70222340571223, 175.00410980408563 220.70222340571223, 176.00410980408563 220.70222340571223, 176.00410980408563 219.70222340571223, 177.00410980408563 218.70222340571223, 179.00410980408563 218.70222340571223, 179.00410980408563 219.70222340571223, 180.00410980408563 218.70222340571223, 182.00410980408563 218.70222340571223, 182.00410980408563 216.70222340571223, 182.00410980408563 215.70222340571223, 184.00410980408563 216.70222340571223, 184.00410980408563 217.70222340571223, 185.00410980408563 217.70222340571223, 185.00410980408563 218.70222340571223, 186.00410980408563 218.70222340571223, 185.00410980408563 218.70222340571223, 186.00410980408563 218.70222340571223, 186.00410980408563 217.70222340571223, 185.00410980408563 217.70222340571223, 185.00410980408563 215.70222340571223, 186.00410980408563 215.70222340571223, 186.00410980408563 218.70222340571223, 187.00410980408563 218.70222340571223, 188.00410980408563 219.70222340571223, 188.00410980408563 220.70222340571223, 189.00410980408563 219.70222340571223, 188.00410980408563 216.70222340571223, 188.00410980408563 218.70222340571223, 188.00410980408563 217.70222340571223, 188.00410980408563 216.70222340571223, 187.00410980408563 216.70222340571223, 187.00410980408563 215.70222340571223, 188.00410980408563 215.70222340571223, 189.00410980408563 214.70222340571223, 190.00410980408563 215.70222340571223, 190.00410980408563 217.70222340571223, 191.00410980408563 219.70222340571223, 191.00410980408563 220.70222340571223, 189.00410980408563 223.70222340571223, 190.00410980408563 223.70222340571223, 189.00410980408563 223.70222340571223, 188.00410980408563 225.70222340571223, 190.00410980408563 223.70222340571223, 192.00410980408563 222.70222340571223, 192.00410980408563 219.70222340571223, 191.00410980408563 216.70222340571223, 192.00410980408563 216.70222340571223, 191.00410980408563 215.70222340571223, 192.00410980408563 215.70222340571223, 195.00410980408563 214.70222340571223, 195.00410980408563 215.70222340571223, 197.00410980408563 215.70222340571223, 195.00410980408563 214.70222340571223, 195.00410980408563 213.70222340571223, 195.00410980408563 212.70222340571223, 193.00410980408563 212.70222340571223, 197.00410980408563 212.70222340571223, 198.00410980408563 213.70222340571223, 199.00410980408563 214.70222340571223, 203.00410980408563 216.70222340571223, 203.00410980408563 217.70222340571223, 199.00410980408563 219.70222340571223 )";
    let wkt = read_wkt(wkt_str);
    let pts = vec![[5.6, 7.9], [5.6, 8.9], [6.6, 8.9], [6.6, 7.9], [5.6, 7.9]];
    let pt_array = vec![[5.6, 7.9, 0.], [5.6, 8.9, 0.], [6.6, 8.9, 0.], [6.6, 7.9, 0.], [5.6, 7.9, 0.]];

    let pts_closed = vec![[5.538, 8.467], [5.498, 8.559], [5.858, 8.987], [6.654, 8.638], [6.549, 8.024], [5.765, 8.082], [5.538, 8.467]];
    let pts_open = vec![[5.538, 8.467], [5.498, 8.559], [5.858, 8.987], [6.654, 8.638], [6.549, 8.024], [5.765, 8.082]];

    let coords: Points = pts.into();
    println!("{}", coords[0]);

    let ln = LineString::new(&wkt.coordinates[0]);
    println!("{}", ln.bbox);
//    for o in ln.chains {
//        println!("{}", o)
//    }
    println!("{}", ln);

}