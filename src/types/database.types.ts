// src/types/database.types.ts

export type Json
  = | string
    | number
    | boolean
    | null
    | { [key: string]: Json | undefined }
    | Json[];

export type Database = {
  public: {
    Tables: {
      med_transactions: {
        Row: {
          id: number;
          created_at: string;
          transaction_date: string; // YYYY-MM-DD
          bill_number: string | null;
          drug_type: string;
          drug_value: number;
          note: string | null;
        };
        Insert: {
          id?: number;
          created_at?: string;
          transaction_date: string;
          bill_number?: string | null;
          drug_type: string;
          drug_value: number;
          note?: string | null;
        };
        Update: {
          id?: number;
          created_at?: string;
          transaction_date?: string;
          bill_number?: string | null;
          drug_type?: string;
          drug_value?: number;
          note?: string | null;
        };
      };
    };
  };
};

export type MedTransaction = Database['public']['Tables']['med_transactions']['Row'];
