use linfa::prelude::*;
use linfa_datasets::iris;
use linfa_trees::DecisionTree;

pub fn do_demo() {
    let _ = create_model_for_dataset();
}

fn create_model_for_dataset() -> Result<(), Box<dyn std::error::Error>> {

    println!("\ncreate_model_for_dataset");
    println!("--------------------------------------");

    // Load the Iris dataset.
    let dataset = iris();   

    println!("Number of samples: {}", dataset.nsamples());
    println!("Number of features: {}", dataset.nfeatures());
    println!("Data records\n: {:?}", dataset.records());

    // Split the Iris dataset into 80% training data, 20% test data. 
    let (train, test) = dataset.split_with_ratio(0.8);
    
    println!("Number of samples in 'training' data: {}", train.nsamples());
    println!("Number of samples in 'test' data:     {}", test.nsamples());

    // Train a DecisionTree model, based on the training data.
    let model = DecisionTree::params().fit(&train)?;

    // Use the model to predict results for the test data.
    let predicted = model.predict(&test);

    let cm = predicted.confusion_matrix(&test)?;
    println!("Accuracy of predicted vs. test data:  {}%", cm.accuracy() * 100.0);
    
    Ok(())
}