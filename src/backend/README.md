// We have Api listening for Agent requests to provide an ListingId, and APIKEY, This will happend with a POST request. with attached Fullname, email and phonenumber.
// An ListingId is created in the database with the status of Open.
// We will then create an APIKEY for the ListingId.
// we will then send a email to the user with his APIKEY and a link to the website where he can upload the files. and A button to Request to be called up again.
// User can then upload pictures in the website frontend. and choose the GPS coordinates of his property.
// We will then store the files in a temporary folder on the server.
// We will then update the status of the Listing to Uploading.

// Create this file that make the temporary files and folders.... All files will have a ttl on 48 hours.
// When files arrive from upload, along with an ListingId, And GPS coordinates. 
// We will store the file in a folder named after the ListingId. 
// we will Update Status to of the listing to Preprocessing.
// We will run Convertion of the files to WebP format. 0.9 quality and ensure that the file is between 1080p and 4k Resolution.
// We will also store the GPS coordinates in the file metadata. 
// We will generate a ImageId for each file and rename each file with this ListingId+ImageId.
// We will store the ListingId Metadata in the files Metadata.
// We will then create a BatchID and Cue it to the ImageAnalysisPipeline.
// We will then update Status of the Listing to Enrichment..
// When the Batch within 24 hours is completed from openAI Image Analysis, we will update the Status of the Listing PostProcessing.
// We will store the result of the ImageAnalysis in the Listing in surrealDB. and Tokenize the results into nested vector embeddings for each field.
// We will enrich each image with the additional information from the ImageAnalysis. and embeddings.
// We will then Run the ImageProcessingPipeline.
// We will then store the result of the ImageProcessingPipeline in the Listing in surrealDB. and Tokenize the results into nested vector embeddings for each field. 
// We will then update the status of the Listing to Storing.
// We will then store the images in the B2 Storage.
// we will create a premade query for retrieval of the entire listing. both for data or files.
// We will then update the status of the Listing to Completed.
// We will then delete the temporary files and folders.
