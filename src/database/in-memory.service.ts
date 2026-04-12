import { Injectable } from '@nestjs/common';

/**
 * In-memory database service for development
 * In production, replace with PostgreSQL/MongoDB
 */
@Injectable()
export class InMemoryDatabaseService {
  private data = new Map<string, Map<string, any>>();

  constructor() {
    this.data.set('splits', new Map());
    this.data.set('payments', new Map());
  }

  getCollection(name: string): Map<string, any> {
    if (!this.data.has(name)) {
      this.data.set(name, new Map());
    }
    return this.data.get(name)!;
  }

  create<T>(collection: string, id: string, data: T): T {
    this.getCollection(collection).set(id, data);
    return data;
  }

  findById<T>(collection: string, id: string): T | null {
    return this.getCollection(collection).get(id) || null;
  }

  findAll<T>(collection: string): T[] {
    return Array.from(this.getCollection(collection).values());
  }

  update<T>(collection: string, id: string, data: Partial<T>): T | null {
    const existing = this.findById<T>(collection, id);
    if (!existing) return null;
    const updated = { ...existing, ...data };
    this.getCollection(collection).set(id, updated);
    return updated;
  }

  delete(collection: string, id: string): boolean {
    return this.getCollection(collection).delete(id);
  }
}
