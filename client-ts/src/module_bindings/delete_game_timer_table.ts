// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

/* eslint-disable */
/* tslint:disable */
// @ts-nocheck
import {
  AlgebraicType,
  AlgebraicValue,
  BinaryReader,
  BinaryWriter,
  CallReducerFlags,
  ConnectionId,
  DbConnectionBuilder,
  DbConnectionImpl,
  DbContext,
  ErrorContextInterface,
  Event,
  EventContextInterface,
  Identity,
  ProductType,
  ProductTypeElement,
  ReducerEventContextInterface,
  SubscriptionBuilderImpl,
  SubscriptionEventContextInterface,
  SumType,
  SumTypeVariant,
  TableCache,
  TimeDuration,
  Timestamp,
  deepEqual,
} from "@clockworklabs/spacetimedb-sdk";
import { DeleteGameTimer } from "./delete_game_timer_type";
import { EventContext, Reducer, RemoteReducers, RemoteTables } from ".";

/**
 * Table handle for the table `delete_game_timer`.
 *
 * Obtain a handle from the [`deleteGameTimer`] property on [`RemoteTables`],
 * like `ctx.db.deleteGameTimer`.
 *
 * Users are encouraged not to explicitly reference this type,
 * but to directly chain method calls,
 * like `ctx.db.deleteGameTimer.on_insert(...)`.
 */
export class DeleteGameTimerTableHandle {
  tableCache: TableCache<DeleteGameTimer>;

  constructor(tableCache: TableCache<DeleteGameTimer>) {
    this.tableCache = tableCache;
  }

  count(): number {
    return this.tableCache.count();
  }

  iter(): Iterable<DeleteGameTimer> {
    return this.tableCache.iter();
  }
  /**
   * Access to the `scheduled_id` unique index on the table `delete_game_timer`,
   * which allows point queries on the field of the same name
   * via the [`DeleteGameTimerScheduledIdUnique.find`] method.
   *
   * Users are encouraged not to explicitly reference this type,
   * but to directly chain method calls,
   * like `ctx.db.deleteGameTimer.scheduled_id().find(...)`.
   *
   * Get a handle on the `scheduled_id` unique index on the table `delete_game_timer`.
   */
  scheduled_id = {
    // Find the subscribed row whose `scheduled_id` column value is equal to `col_val`,
    // if such a row is present in the client cache.
    find: (col_val: bigint): DeleteGameTimer | undefined => {
      for (let row of this.tableCache.iter()) {
        if (deepEqual(row.scheduled_id, col_val)) {
          return row;
        }
      }
    },
  };
  /**
   * Access to the `game_id` unique index on the table `delete_game_timer`,
   * which allows point queries on the field of the same name
   * via the [`DeleteGameTimerGameIdUnique.find`] method.
   *
   * Users are encouraged not to explicitly reference this type,
   * but to directly chain method calls,
   * like `ctx.db.deleteGameTimer.game_id().find(...)`.
   *
   * Get a handle on the `game_id` unique index on the table `delete_game_timer`.
   */
  game_id = {
    // Find the subscribed row whose `game_id` column value is equal to `col_val`,
    // if such a row is present in the client cache.
    find: (col_val: number): DeleteGameTimer | undefined => {
      for (let row of this.tableCache.iter()) {
        if (deepEqual(row.game_id, col_val)) {
          return row;
        }
      }
    },
  };

  onInsert = (cb: (ctx: EventContext, row: DeleteGameTimer) => void) => {
    return this.tableCache.onInsert(cb);
  }

  removeOnInsert = (cb: (ctx: EventContext, row: DeleteGameTimer) => void) => {
    return this.tableCache.removeOnInsert(cb);
  }

  onDelete = (cb: (ctx: EventContext, row: DeleteGameTimer) => void) => {
    return this.tableCache.onDelete(cb);
  }

  removeOnDelete = (cb: (ctx: EventContext, row: DeleteGameTimer) => void) => {
    return this.tableCache.removeOnDelete(cb);
  }

  // Updates are only defined for tables with primary keys.
  onUpdate = (cb: (ctx: EventContext, oldRow: DeleteGameTimer, newRow: DeleteGameTimer) => void) => {
    return this.tableCache.onUpdate(cb);
  }

  removeOnUpdate = (cb: (ctx: EventContext, onRow: DeleteGameTimer, newRow: DeleteGameTimer) => void) => {
    return this.tableCache.removeOnUpdate(cb);
  }}
