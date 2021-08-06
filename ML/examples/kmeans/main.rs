use ML::*;

// Import the linfa prelude and KMeans algorithm
use linfa::prelude::*;
use linfa_clustering::KMeans;

// We'll build our dataset on our own using ndarray and rand
use ndarray::prelude::*;
use rand::prelude::*;

// Import the plotters crate to create the scatter plot
use plotters::prelude::*;

fn main() {

    //we're going to create squares filled with random points and each square is defined by a center point, edge length and number of points 
    let square_1: Array2<f32> = create_square([7.0, 5.0], 1.0, 150); // Cluster 1
    let square_2 = create_square([2.0, 2.0], 2.0, 150); // Cluster 2
    let square_3 = create_square([3.0, 8.0], 1.0, 150); // Cluster 3

    //// We also need to create a background of points that will act as noise
    let square_4 = create_square([5.0, 5.0], 9.0, 300); 

    let _data = ndarray::stack(
    Axis(0),
    &[
    square_1.view(),
    square_2.view(),
    square_3.view(),
    square_4.view(),
    ],
    )
    .expect("An error occurred while stacking the dataset");
    println!("{}",_data);
}