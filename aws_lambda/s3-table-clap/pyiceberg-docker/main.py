#!/usr/bin/env python3
"""
PyIceberg AWS Integration Script

This script provides functionality to interact with Apache Iceberg tables in AWS S3
using the PyIceberg library. It demonstrates table creation, data insertion, querying,
and time travel operations.
"""

from pyiceberg.catalog import load_catalog
import os
import pyarrow as pa
import pandas as pd
from pyiceberg.expressions import EqualTo
import boto3
import json
import argparse
from botocore.exceptions import ProfileNotFound
from datetime import datetime
from tabulate import tabulate

# Constants
DEFAULT_REGION = 'us-east-2'
DEFAULT_CATALOG = 'S3TablesCatalog'
DEFAULT_DATABASE = 'myblognamespace'
DEFAULT_S3_BUCKET = 'pyiceberg-blog-bucket'

def format_timestamp(timestamp_ms: int) -> str:
    """
    Convert millisecond timestamp to readable datetime string.
    """
    return datetime.fromtimestamp(timestamp_ms / 1000).strftime('%Y-%m-%d %H:%M:%S')

def get_aws_details() -> tuple:
    """
    Retrieve AWS account ID and region from local AWS configuration.
    """
    try:
        sts_client = boto3.client('sts')
        session = boto3.session.Session()
        account_id = sts_client.get_caller_identity()["Account"]
        region = session.region_name or DEFAULT_REGION
        return account_id, region
    except Exception as e:
        print(f"Error retrieving AWS details: {e}")
        return None, None

def format_databases(databases: list) -> None:
    """
    Format and print database list in a readable way.
    """
    print("\n Available Databases:")
    db_data = [{'Database': db[0]} for db in databases]
    print(tabulate(db_data, headers='keys', tablefmt='grid', showindex=True))

def format_tables(tables: list, database: str) -> None:
    """
    Format and print table list in a readable way.
    """
    print(f"\n Tables in {database}:")
    table_data = [{'Table Name': table[1]} for table in tables]
    print(tabulate(table_data, headers='keys', tablefmt='grid', showindex=True))

def create_customer_schema() -> pa.Schema:
    """
    Create and return the PyArrow schema for customer table.
    """
    return pa.schema([
        pa.field('c_salutation', pa.string()),
        pa.field('c_preferred_cust_flag', pa.string()),
        pa.field('c_first_sales_date_sk', pa.int32()),
        pa.field('c_customer_sk', pa.int32()),
        pa.field('c_first_name', pa.string()),
        pa.field('c_email_address', pa.string())
    ])

def get_sample_customer_data() -> dict:
    """
    Return sample customer data for testing.
    """
    return {
        "c_salutation": "Ms",
        "c_preferred_cust_flag": "NULL",
        "c_first_sales_date_sk": 2452736,
        "c_customer_sk": 1234,
        "c_first_name": "Mickey",
        "c_email_address": "mickey@email.com"
    }

def highlight_preferred_flag(df: pd.DataFrame) -> pd.DataFrame:
    """
    Highlight the c_preferred_cust_flag column in the output.
    """
    return pd.DataFrame(
        [[' ' + str(val) if col == 'c_preferred_cust_flag' else str(val) 
          for col, val in row.items()] 
         for _, row in df.iterrows()],
        columns=df.columns
    )

def print_table_data(df: pd.DataFrame, title: str, highlight_flag: bool = False) -> None:
    """
    Print table data in a formatted way with optional highlighting.
    
    Args:
        df (pd.DataFrame): DataFrame to print
        title (str): Title for the data
        highlight_flag (bool): Whether to highlight the preferred_cust_flag column
    """
    print(f"\n {title}")
    
    if highlight_flag:
        # Create a copy to avoid modifying the original DataFrame
        display_df = df.copy()
        
        # Reorder columns to put c_preferred_cust_flag near the beginning
        if 'c_preferred_cust_flag' in display_df.columns:
            cols = ['c_first_name', 'c_preferred_cust_flag'] + \
                   [col for col in display_df.columns if col not in [
                       'c_first_name', 'c_preferred_cust_flag'
                   ]]
            display_df = display_df[cols]
            
            # Add visual indicator for the flag
            display_df['c_preferred_cust_flag'] = ' ' + display_df['c_preferred_cust_flag'].astype(str)
            
            # Add a header separator
            print("=" * 80)
            print("Focus on c_preferred_cust_flag column for changes")
            print("=" * 80)
    else:
        display_df = df

    print(tabulate(display_df, headers='keys', tablefmt='grid', showindex=False))

def print_snapshot_info(snapshots: list) -> None:
    """
    Print snapshot information in a formatted table.
    """
    if not snapshots:
        print("\n No snapshots found for this table")
        return

    snapshot_data = []
    for snapshot in snapshots:
        snapshot_info = {
            'Snapshot ID': snapshot.snapshot_id,
            'Timestamp': format_timestamp(snapshot.timestamp_ms),
            'Manifest List': snapshot.manifest_list,
            'Schema ID': snapshot.schema_id,
            'Summary': snapshot.summary if hasattr(snapshot, 'summary') else 'N/A'
        }
        snapshot_data.append(snapshot_info)

    print("\n Snapshot History:")
    print(tabulate(snapshot_data, headers='keys', tablefmt='grid'))

def initialize_catalog(catalog_name: str, account_id: str, s3tablebucketname: str, region: str):
    """
    Initialize and return the REST catalog.
    """
    return load_catalog(
        catalog_name,
        **{
            "type": "rest",
            "warehouse": f"{account_id}:s3tablescatalog/{s3tablebucketname}",
            "uri": f"https://glue.{region}.amazonaws.com/iceberg",
            "rest.sigv4-enabled": "true",
            "rest.signing-name": "glue",
            "rest.signing-region": region
        }
    )

def hydrates3tablesbucket(catalog_name: str, s3tablebucketname: str, database_name: str, 
                         table_name: str, account_id: str, region: str) -> None:
    """
    Handle Iceberg table operations including creation, data insertion, and querying.
    """
    try:
        print("\n" + "="*50)
        print(" Initializing Iceberg Table Operations")
        print("="*50)
        print(f" Catalog    : {catalog_name}")
        print(f" S3 Bucket  : {s3tablebucketname}")
        print(f" Database   : {database_name}")
        print(f" Table      : {table_name}")
        print("="*50)

        # Initialize REST catalog
        rest_catalog = initialize_catalog(catalog_name, account_id, s3tablebucketname, region)

        # List available databases and tables
        databases = rest_catalog.list_namespaces()
        tables = rest_catalog.list_tables(namespace=database_name)
        
        format_databases(databases)
        format_tables(tables, database_name)

        # Create table with schema
        print("\n Creating table schema...")
        my_schema = create_customer_schema()
        
        # Check if table exists before creating
        try:
            rest_catalog.create_table(
                identifier=f"{database_name}.{table_name}",
                schema=my_schema
            )
            print("Table created successfully")
        except Exception as e:
            print(f"ℹ Table creation note: {str(e)}")

        # Load the table
        table = rest_catalog.load_table(f"{database_name}.{table_name}")
        print(f" Table schema: {table.schema}")

        # Insert sample data
        print("\n Inserting sample data...")
        sample_data = get_sample_customer_data()
        df = pa.Table.from_pylist([sample_data], schema=my_schema)
        table.append(df)
        print(" Sample data inserted successfully")

        # Query and display initial data
        print("\n Querying initial data...")
        tabledata = table.scan(
            row_filter=EqualTo("c_first_name", "Mickey"),
            limit=10
        ).to_pandas()
        print_table_data(tabledata, "Initial Data - Check preferred_cust_flag value", highlight_flag=True)

        # Update customer flag
        print("\n Updating customer flag...")
        print("Changing c_preferred_cust_flag from 'NULL' to 'N'")
        condition = tabledata['c_preferred_cust_flag'] == 'NULL'
        tabledata.loc[condition, 'c_preferred_cust_flag'] = 'N'
        df2 = pa.Table.from_pandas(tabledata, schema=my_schema)
        table.overwrite(df2)

        # Display updated data
        updated_tabledata = table.scan(
            row_filter=EqualTo("c_first_name", "Mickey"),
            limit=10
        ).to_pandas()
        print_table_data(updated_tabledata, "Updated Data - Notice the changed preferred_cust_flag", highlight_flag=True)

        # Add a summary of changes
        print("\n Summary of Changes:")
        print("=" * 80)
        print("c_preferred_cust_flag modifications:")
        print(f"Before: NULL")
        print(f"After:  N")
        print("=" * 80)
        
        # Time Travel Operations
        print("\n Performing Time Travel Operations...")
        customer_snapshots = table.snapshots()
        print_snapshot_info(customer_snapshots)

        if customer_snapshots:
            latest_snapshot = customer_snapshots[0]
            print(f"\n Retrieving data from snapshot {latest_snapshot.snapshot_id}")
            print(f" Snapshot timestamp: {format_timestamp(latest_snapshot.timestamp_ms)}")
            
            customer_snapshotdata = table.scan(snapshot_id=latest_snapshot.snapshot_id).to_arrow()
            snapshot_df = customer_snapshotdata.to_pandas()
            print_table_data(snapshot_df, "Snapshot Data")

            print("\n Time Travel Summary:")
            print(f"Total snapshots available: {len(customer_snapshots)}")
            if len(customer_snapshots) > 1:
                print(f"Earliest snapshot: {format_timestamp(customer_snapshots[-1].timestamp_ms)}")
            print(f"Latest snapshot: {format_timestamp(customer_snapshots[0].timestamp_ms)}")

    except Exception as e:
        print(f"\n Error during table operations: {str(e)}")
        raise

    print("\n All operations completed successfully!")

def main():
    """
    Main function to handle command line arguments and execute table operations.
    """
    try:
        parser = argparse.ArgumentParser(
            description='Process Apache Iceberg table operations with AWS integration'
        )
        
        parser.add_argument('--catalog', default=DEFAULT_CATALOG, help='Catalog name')
        parser.add_argument('--s3tablebucket', default=DEFAULT_S3_BUCKET, help='S3 Table Bucket name')
        parser.add_argument('--database', default=DEFAULT_DATABASE, help='Database name')
        parser.add_argument('--table', required=True, help='Table name')

        args = parser.parse_args()

        # Get AWS account details
        account_id, region = get_aws_details()

        if account_id and region:
            print(f"AWS Configuration - Account: {account_id}, Region: {region}")
            
            hydrates3tablesbucket(
                args.catalog,
                args.s3tablebucket,
                args.database,
                args.table,
                account_id,
                region
            )
        else:
            print(" Failed to retrieve AWS details. Please verify AWS configuration.")

    except Exception as e:
        print(f" Error in main execution: {str(e)}")
        raise

if __name__ == '__main__':
    main()